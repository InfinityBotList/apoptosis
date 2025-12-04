use super::require::{AssetRequirer, FilesystemWrapper};
use mlua_scheduler::{ReturnTracker, TaskManager, taskmgr::Hooks};
use mluau::{Compiler, prelude::*};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    time::{Duration, Instant},
};

use crate::{Error, service::kittycat::kittycat_base_tab};

// Core modules
type CoreModule<'a> = (&'a str, fn(&Lua) -> LuaResult<LuaTable>);
const CORE_MODULES: &[CoreModule] = &[("@omniplex/kittycat", kittycat_base_tab)];

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct VmCreateOpts {
    disable_task_lib: bool,
    time_limit: Option<Duration>,
    give_time: Duration,
}

pub type OnBrokenFunc = Box<dyn Fn()>;
pub type OnDropFunc = Box<dyn Fn()>;

struct FunctionCache(RefCell<HashMap<String, LuaFunction>>);

/// The internal VM structure used by VmThread
#[allow(dead_code)]
pub struct Vm {
    /// The Luau VM instance
    lua: Rc<RefCell<Option<Lua>>>,

    /// The Luau compiler instance
    compiler: Compiler,

    /// The task manager for scheduling Luau tasks
    scheduler: TaskManager,

    /// The last time the VM executed a script
    last_execution_time: Rc<Cell<Option<Instant>>>,

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

    /// The underlying filesystem wrapper
    fsw: FilesystemWrapper,

    /// Function cache for the VM
    function_cache: FunctionCache,
}

#[allow(dead_code)]
impl Vm {
    pub async fn new(opts: VmCreateOpts, fsw: FilesystemWrapper) -> LuaResult<Self> {
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

        let controller = AssetRequirer::new(fsw.clone(), "srv".to_string(), lua.globals());

        lua.globals()
            .set("require", lua.create_require_function(controller)?)?;

        lua.globals().set(
            "warn",
            lua.create_function(|_lua, msg: String| {
                log::warn!("{msg}");
                Ok(())
            })?,
        )?;

        Ok(Self {
            lua: Rc::new(RefCell::new(Some(lua))),
            compiler,
            scheduler,
            time_limit,
            execution_stop_time,
            broken,
            on_broken: Rc::new(RefCell::new(None)),
            sandboxed: Rc::new(Cell::new(false)),
            fsw,
            function_cache: FunctionCache(RefCell::new(HashMap::new())),
            last_execution_time: Rc::new(Cell::new(None)),
        })
    }

    /// Updates the last execution time
    pub fn update_last_execution_time(&self, time: Instant) {
        self.last_execution_time.set(Some(time));

        // Update the execution stop time as well
        self.execution_stop_time
            .set(self.time_limit.get().map(|limit| time + limit));
    }

    pub fn mark_broken(&self, broken: bool) -> Result<(), Error> {
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
    pub fn set_on_broken(&self, callback: OnBrokenFunc) {
        log::debug!("Setting on_broken callback");
        self.on_broken.borrow_mut().replace(callback);
    }

    /// Registers a callback to be called when the vm is dropped/closed
    pub fn set_on_close(&self, f: OnDropFunc) {
        let lua_opt = self.lua.borrow();
        if let Some(ref lua) = *lua_opt {
            lua.set_on_close(f);
        }
    }

    /// Sandboxes the VM, making globals readonly etc.
    pub fn sandbox(&mut self) -> Result<(), LuaError> {
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
    pub fn memory_usage(&self) -> usize {
        let Some(ref lua) = *self.lua.borrow() else {
            return 0;
        };
        lua.used_memory()
    }

    /// Sets a memory limit for the vm
    ///
    /// The memory limit is set in bytes and will be enforced by the lua vm itself
    /// (e.g. using mlua)
    pub fn set_memory_limit(&self, limit: usize) -> Result<usize, LuaError> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        lua.set_memory_limit(limit)
    }

    /// Runs a script in the vm
    pub async fn run<T: IntoLuaMulti>(&self, path: &str, args: T) -> Result<SpawnResult, LuaError> {
        let code = self
            .fsw
            .get_file(path.to_string())
            .map_err(|e| LuaError::RuntimeError(format!("Failed to load asset '{path}': {e}")))?;

        self.update_last_execution_time(Instant::now()); // Update last execution time

        let thread = {
            let Some(ref lua) = *self.lua.borrow() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };

            let mut cache = self.function_cache.0.borrow_mut();
            let f = if let Some(f) = cache.get(path) {
                f.clone() // f is cheap to clone
            } else {
                let bytecode = self.compiler.compile(code)?;

                let function = match lua
                    .load(&bytecode)
                    .set_name(path.to_string())
                    .set_mode(mluau::ChunkMode::Binary) // Ensure auto-detection never selects binary mode
                    //.set_environment(self.global_table.clone())
                    .into_function()
                {
                    Ok(f) => f,
                    Err(e) => {
                        // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                        if let LuaError::MemoryError(_) = e {
                            // Mark VM as broken
                            self.mark_broken(true)
                                .map_err(|e| LuaError::external(e.to_string()))?;
                        }
                        return Err(e);
                    }
                };

                cache.insert(path.to_string(), function.clone());
                function
            };

            match lua.create_thread(f) {
                Ok(f) => f,
                Err(e) => {
                    // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                    if let LuaError::MemoryError(_) = e {
                        // Mark VM as broken
                        self.mark_broken(true)
                            .map_err(|e| LuaError::external(e.to_string()))?;
                    }

                    return Err(e);
                }
            }
        };

        // Update last_execution_time
        self.update_last_execution_time(std::time::Instant::now());

        let args = {
            let Some(ref lua) = *self.lua.borrow() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };
            match args.into_lua_multi(lua) {
                Ok(a) => a,
                Err(e) => {
                    // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                    if let LuaError::MemoryError(_) = e {
                        // Mark VM as broken
                        self.mark_broken(true)
                            .map_err(|e| LuaError::external(e.to_string()))?;
                    }

                    return Err(e);
                }
            }
        };

        let res = self.scheduler.spawn_thread_and_wait(thread, args).await?;

        // Do a GC
        {
            let Some(ref lua) = *self.lua.borrow() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };

            lua.gc_collect()?;
            lua.gc_collect()?; // Twice to ensure we get all the garbage
        }

        // Now unwrap it
        let res = match res {
            Some(Ok(res)) => Some(res),
            Some(Err(e)) => {
                // Mark memory error'd VMs as broken automatically to avoid user grief/pain
                if let LuaError::MemoryError(_) = e {
                    // Mark VM as broken
                    self.mark_broken(true)
                        .map_err(|e| LuaError::external(e.to_string()))?;
                }

                return Err(e);
            }
            None => None,
        };

        Ok(SpawnResult::new(res))
    }

    /// Closes the VM, dropping the underlying Lua instance
    pub fn close(&self) -> Result<(), Error> {
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

pub struct SpawnResult {
    result: Option<LuaMultiValue>,
}

#[allow(dead_code)]
impl SpawnResult {
    pub(crate) fn new(result: Option<LuaMultiValue>) -> Self {
        Self { result }
    }

    /// Unwraps the spawn result into a LuaMultiValue
    pub fn into_inner(self) -> Option<LuaMultiValue> {
        self.result
    }

    /// Converts the spawn result into a specific serializable type
    pub fn into_value<T: serde::de::DeserializeOwned>(
        self,
        vm: &Vm,
    ) -> Result<Option<T>, LuaError> {
        if let Some(mut mv) = self.result {
            let Some(value) = mv.pop_front() else {
                return Ok(None);
            };

            let Some(ref lua) = *vm.lua.borrow() else {
                return Err(LuaError::RuntimeError(
                    "Lua instance is no longer valid".to_string(),
                ));
            };

            let value = lua.from_value(value)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub fn into_must_value<T: serde::de::DeserializeOwned>(self, vm: &Vm) -> Result<T, LuaError> {
        match self.into_value::<T>(vm)? {
            Some(v) => Ok(v),
            None => Err(LuaError::RuntimeError(
                "Expected value but got none".to_string(),
            )),
        }
    }
}
