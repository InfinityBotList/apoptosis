use mlua_scheduler::{ReturnTracker, TaskManager, taskmgr::Hooks};
use mluau::{Compiler, prelude::*};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::{Duration, Instant},
};

use crate::service::kittycat::kittycat_base_tab;

// Core modules
type CoreModule<'a> = (&'a str, fn(&Lua) -> LuaResult<LuaTable>);
const CORE_MODULES: &[CoreModule] = &[("@omniplex/kittycat", kittycat_base_tab)];

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize, Default)]
pub(super) struct VmCreateOpts {
    disable_task_lib: bool,
    time_limit: Option<Duration>,
    give_time: Duration,
}

type OnBrokenFunc = Box<dyn Fn()>;
pub(super) type Error = Box<dyn std::error::Error + Send + Sync>;

/// The internal VM structure used by VmThread
#[allow(dead_code)]
pub(super) struct Vm {
    /// The Luau VM instance
    lua: Rc<RefCell<Option<Lua>>>,

    /// The Luau compiler instance
    compiler: Compiler,

    /// The task manager for scheduling Luau tasks
    scheduler: TaskManager,

    /// The time limit for execution
    time_limit: Rc<Cell<Option<Duration>>>,

    /// The time the execution should stop at
    ///
    /// Automatically calculated (usually) from time_limit and last_execution_time
    ///
    /// Scheduler resumes may extend this time
    execution_stop_time: Rc<Cell<Option<Instant>>>,

    /// Is the vm instance 'broken' or not
    broken: Rc<Cell<bool>>,

    /// A function to be called if the vm is marked as broken
    on_broken: Rc<RefCell<Option<OnBrokenFunc>>>,

    /// Whether the VM is sandboxed or not
    sandboxed: Rc<Cell<bool>>,
}

#[allow(dead_code)]
impl Vm {
    pub(super) async fn new(opts: VmCreateOpts) -> LuaResult<Self> {
        let lua = Lua::new_with(
            LuaStdLib::ALL_SAFE,
            LuaOptions::new()
                .catch_rust_panics(true)
                .disable_error_userdata(true),
        )?;

        let compiler = Compiler::new()
            .set_optimization_level(2)
            .set_type_info_level(1);

        lua.set_compiler(compiler.clone());

        // Set up the scheduler with time limit handling
        let time_limit = Rc::new(Cell::new(opts.time_limit));
        let execution_stop_time = match opts.time_limit {
            Some(limit) => Rc::new(Cell::new(Some(Instant::now() + limit))),
            None => Rc::new(Cell::new(None)),
        };
        let scheduler = TaskManager::new(
            &lua,
            ReturnTracker::new(),
            Rc::new(SchedulerHook {
                execution_stop_time: execution_stop_time.clone(),
                give_time: opts.give_time,
            }),
        )
        .await
        .map_err(|e| LuaError::external(format!("Failed to create task manager: {e:?}")))?;

        if !opts.disable_task_lib {
            lua.globals()
                .set("task", mlua_scheduler::userdata::task_lib(&lua)?)?;
        }

        let broken = Rc::new(Cell::new(false));
        let broken_ref = broken.clone();

        let execution_stop_time_ref = execution_stop_time.clone();
        lua.set_interrupt(move |_lua| {
            // If the vm is broken, yield the lua vm immediately
            let broken = broken_ref.get();
            if broken {
                return Ok(LuaVmState::Yield);
            }

            #[allow(clippy::collapsible_if)]
            if let Some(limit) = execution_stop_time_ref.get() {
                if Instant::now() > limit {
                    return Err(LuaError::RuntimeError(
                        "Script execution time limit exceeded".to_string(),
                    ));
                }
            }

            Ok(LuaVmState::Continue)
        });

        // Load core modules
        for (mod_name, mod_loader) in CORE_MODULES {
            lua.register_module(mod_name, mod_loader(&lua)?)?;
        }

        Ok(Self {
            lua: Rc::new(RefCell::new(Some(lua))),
            compiler,
            scheduler,
            time_limit,
            execution_stop_time,
            broken,
            on_broken: Rc::new(RefCell::new(None)),
            sandboxed: Rc::new(Cell::new(false)),
        })
    }

    pub(super) fn mark_broken(&self, broken: bool) -> Result<(), Error> {
        log::debug!("Marking vm as broken");
        let mut stat = Ok(());
        match self.close() {
            Ok(_) => {}
            Err(e) => {
                self.broken.set(true); // Ensure vm is still at least marked as broken
                stat = Err(e); // Set return value to the error
            }
        };

        // Call the on_broken callback if the vm is marked as broken
        //
        // This must be called regardless of if close failed or not to ensure at least
        // other handles are closed
        #[allow(clippy::collapsible_if)]
        if broken {
            if let Some(ref on_broken) = *self.on_broken.borrow() {
                on_broken();
            }
        }

        stat
    }

    /// Registers a callback to be called when the vm is marked as broken
    pub(super) fn set_on_broken(&self, callback: OnBrokenFunc) {
        log::debug!("Setting on_broken callback");
        self.on_broken.borrow_mut().replace(callback);
    }

    /// Sandboxes the VM, making globals readonly etc.
    pub(super) fn sandbox(&mut self) -> Result<(), LuaError> {
        if self.sandboxed.get() {
            return Ok(());
        }

        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        lua.sandbox(true)?;
        lua.globals().set_readonly(true);
        self.sandboxed.set(true);
        Ok(())
    }

    /// Returns the current memory usage of the vm
    ///
    /// Returns `0` if the lua vm is not valid
    pub(super) fn memory_usage(&self) -> usize {
        let Some(ref lua) = *self.lua.borrow() else {
            return 0;
        };
        lua.used_memory()
    }

    /// Sets a memory limit for the vm
    ///
    /// The memory limit is set in bytes and will be enforced by the lua vm itself
    /// (e.g. using mlua)
    pub(super) fn set_memory_limit(&self, limit: usize) -> Result<usize, LuaError> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        lua.set_memory_limit(limit)
    }

    pub(super) fn close(&self) -> Result<(), Error> {
        self.broken.set(true); // Mark the vm as broken if it is closed

        {
            if let Some(ref lua) = *self.lua.borrow_mut() {
                {
                    // Ensure strong_count == 1
                    if lua.strong_count() > 1 {
                        log::warn!("Lua VM is still in use and may not be closed immediately");
                    }
                }
            } else {
                return Ok(()); // Lua VM is already closed
            }
        }

        *self.lua.borrow_mut() = None; // Drop the Lua VM
        self.broken.set(true); // Mark the vm as broken if it is closed

        Ok(())
    }
}

/// Hook to manage Luau scheduler execution time
///
/// Useful for protecting against runaway code
struct SchedulerHook {
    execution_stop_time: Rc<Cell<Option<Instant>>>,
    give_time: Duration,
}

impl Hooks for SchedulerHook {
    fn on_resume(&self, _thread: &mluau::Thread) {
        match self.execution_stop_time.get() {
            Some(curr_stop) => {
                // We need to give the thread some time to run

                // If current stopping time is less than now + give_time, meaning
                // the thread wouldn't be able to run for at least give_time,
                // extend the time a bit
                if curr_stop < Instant::now() + self.give_time {
                    // Extend the time a bit
                    self.execution_stop_time
                        .set(Some(Instant::now() + self.give_time));
                }
            }
            None => {
                self.execution_stop_time.set(None);
            }
        }
    }
}
