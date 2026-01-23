use mluau_require::{AssetRequirer, FilesystemWrapper};
use mlua_scheduler::{ReturnTracker, TaskManager, taskmgr::Hooks};
use mluau::prelude::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Instant,
};

use crate::service::{
    kittycat::kittycat_base_tab, 
    json::json_tab, 
    luacore::{
        datetime::datetime_tab,
        datamgmt::datamgmt_tab,
        interop::interop_plugin,
        luau::luau_plugin,
        typesext::typesext_plugin
    }
};

// Core modules
type CoreModule<'a> = (&'a str, fn(&Lua) -> LuaResult<LuaTable>);
const CORE_MODULES: &[CoreModule] = &[
    ("@omniplex-rust/kittycat", kittycat_base_tab),
    ("@omniplex-rust/json", json_tab),
    ("@omniplex-rust/datetime", datetime_tab),
    ("@omniplex-rust/datamgmt", datamgmt_tab),
    ("@omniplex-rust/interop", interop_plugin),
    ("@omniplex-rust/luau", luau_plugin),
    ("@omniplex-rust/typesext", typesext_plugin),
];

/// A function to be called when the runtime is marked as broken
pub type OnBrokenFunc = Box<dyn Fn()>;

/// Auxillary options for the creation of a runtime
#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct RuntimeCreateOpts {
    pub disable_task_lib: bool,
    pub time_limit: Option<std::time::Duration>,
    pub give_time: std::time::Duration,
    //pub time_slice: Option<std::time::Duration>,
}

pub struct SchedulerHook {
    execution_stop_time: Rc<Cell<Option<std::time::Instant>>>,
    give_time: std::time::Duration,
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
                    self.execution_stop_time.set(Some(Instant::now() + self.give_time));
                }
            }
            None => {
                self.execution_stop_time.set(None);
            }
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Vm {
    /// The lua vm itself
    pub(super) lua: Rc<RefCell<Option<Lua>>>,

    /// The lua compiler itself
    compiler: mluau::Compiler,

    /// The vm scheduler
    scheduler: TaskManager,

    /// Is the runtime instance 'broken' or not
    broken: Rc<Cell<bool>>,

    /// A function to be called if the runtime is marked as broken
    on_broken: Rc<RefCell<Option<OnBrokenFunc>>>,

    /// The last time the VM executed a script
    last_execution_time: Rc<Cell<Option<Instant>>>,

    /// The time limit for execution
    time_limit: Rc<Cell<Option<std::time::Duration>>>,

    /// The time the execution should stop at
    /// 
    /// Automatically calculated (usually) from time_limit and last_execution_time
    /// 
    /// Scheduler resumes may extend this time
    execution_stop_time: Rc<Cell<Option<Instant>>>,

    /// The base global table
    global_table: LuaTable,

    /// The proxy require function
    proxy_require: LuaFunction,

    /// runtime creation options
    opts: RuntimeCreateOpts,
}

#[allow(dead_code)]
impl Vm {
    pub async fn new<
        FS: mluau_require::vfs::FileSystem + 'static,
    >(
        opts: RuntimeCreateOpts,
        vfs: FS,
    ) -> Result<Self, LuaError> {
        let lua = Lua::new_with(
            LuaStdLib::ALL_SAFE,
            LuaOptions::new()
                .catch_rust_panics(true)
                .disable_error_userdata(true),
        )?;

        let compiler = mluau::Compiler::new()
            .set_optimization_level(2)
            .set_type_info_level(1);

        lua.set_compiler(compiler.clone());

        let time_limit = Rc::new(Cell::new(opts.time_limit));
        let execution_stop_time = match opts.time_limit {
            Some(limit) => Rc::new(Cell::new(Some(Instant::now() + limit))),
            None => Rc::new(Cell::new(None)),
        };
        let scheduler = TaskManager::new(&lua, ReturnTracker::new(), Rc::new(SchedulerHook {
            execution_stop_time: execution_stop_time.clone(),
            give_time: opts.give_time
        }))
        .await
        .map_err(|e| {
            LuaError::external(format!(
                "Failed to create task manager: {e:?}"
            ))
        })?;

        if !opts.disable_task_lib {
            lua.globals()
                .set("task", mlua_scheduler::userdata::task_lib(&lua)?)?;
        }

        let broken = Rc::new(Cell::new(false));
        let broken_ref = broken.clone();
        let last_execution_time: Rc<Cell<Option<Instant>>> = Rc::new(Cell::new(None));
        //let time_slice = Rc::new(Cell::new(opts.time_slice));

        let execution_stop_time_ref = execution_stop_time.clone();
        //let time_slice_ref = time_slice.clone();
        lua.set_interrupt(move |_lua| {
            // If the runtime is broken, yield the lua vm immediately
            let broken = broken_ref.get();
            if broken {
                return Ok(LuaVmState::Yield);
            }
            
            if let Some(limit) = execution_stop_time_ref.get() {
                if Instant::now() > limit {
                    return Err(LuaError::RuntimeError(
                        "Script execution time limit exceeded".to_string(),
                    ));
                }
            }

            Ok(LuaVmState::Continue)
        });

        // Setup require function
        let global_table = proxy_global(&lua)?;
        let controller = AssetRequirer::new(FilesystemWrapper::new(vfs), "main".to_string(), global_table.clone());
        let require = lua.create_require_function(controller)?;
        global_table
            .set("require", require)?;

        let proxy_require = lua.load("return require(...)")
            .set_environment(global_table.clone())
            .set_name("/init.luau")
            .set_mode(mluau::ChunkMode::Text)
            .try_cache()
            .into_function()?;

        // Now, sandbox the lua vm
        lua.sandbox(true)?;
        lua.globals().set_readonly(true);
        lua.globals().set_safeenv(true);

        // Load core modules
        for (name, init_func) in CORE_MODULES {
            lua.register_module(name, init_func(&lua)?)?;
        }

        Ok(Self {
            global_table,
            lua: Rc::new(RefCell::new(Some(lua))),
            compiler,
            scheduler,
            broken,
            on_broken: Rc::new(RefCell::new(None)),
            last_execution_time,
            time_limit,
            execution_stop_time,
            //time_slice,
            opts,
            proxy_require
        })
    }

    /// Returns the scheduler
    pub fn scheduler(&self) -> &TaskManager {
        log::debug!("Getting scheduler");
        &self.scheduler
    }

    /// Returns the last execution time
    ///
    /// This may be None if the VM has not executed a script yet
    pub fn last_execution_time(&self) -> Option<Instant> {
        log::debug!("Getting last execution time");
        self.last_execution_time.get()
    }

    /// Updates the last execution time
    pub fn update_last_execution_time(&self, time: Instant) {
        log::debug!("Updating last execution time");
        self.last_execution_time.set(Some(time));

        // Update the execution stop time as well
        self.execution_stop_time.set(self.time_limit.get().map(|limit| time + limit));
    }

    /// Returns the time limit for execution
    pub fn time_limit(&self) -> Option<std::time::Duration> {
        self.time_limit.get()
    }

    /// Sets the time limit for execution
    pub fn set_time_limit(&self, limit: Option<std::time::Duration>) {
        self.time_limit.set(limit);
    }

    /// Returns whether the runtime is broken or not
    pub fn is_broken(&self) -> bool {
        log::debug!("Getting if runtime is broken");
        self.broken.get()
    }

    /// Returns the runtime creation options
    pub fn opts(&self) -> &RuntimeCreateOpts {
        log::debug!("Getting runtime creation options");
        &self.opts
    }

    /// Sets the runtime to be broken. This will also attempt to close the lua vm but
    /// will still call the on_broken callback if it is set regardless of return of close
    ///
    /// It is a logic error to call this function while holding a reference to the lua vm
    pub fn mark_broken(&self, broken: bool) -> Result<(), crate::Error> {
        log::debug!("Marking runtime as broken");
        let mut stat = Ok(());
        match self.close() {
            Ok(_) => {}
            Err(e) => {
                self.broken.set(true); // Ensure runtime is still at least marked as broken
                stat = Err(e); // Set return value to the error
            }
        };

        // Call the on_broken callback if the runtime is marked as broken
        //
        // This must be called regardless of if close failed or not to ensure at least
        // other handles are closed
        if broken {
            if let Some(ref on_broken) = *self.on_broken.borrow() {
                on_broken();
            }
        }

        stat
    }

    /// Returns if a on_broken callback is set
    pub fn has_on_broken(&self) -> bool {
        log::debug!("Getting if on_broken callback is set");
        self.on_broken.borrow().is_some()
    }

    /// Registers a callback to be called when the runtime is marked as broken
    pub fn set_on_broken(&self, callback: OnBrokenFunc) {
        log::debug!("Setting on_broken callback");
        self.on_broken.borrow_mut().replace(callback);
    }

    /// Returns the current memory usage of the runtime
    ///
    /// Returns `0` if the lua vm is not valid
    pub fn memory_usage(&self) -> usize {
        let Some(ref lua) = *self.lua.borrow() else {
            return 0;
        };
        lua.used_memory()
    }

    /// Sets a memory limit for the runtime
    ///
    /// The memory limit is set in bytes and will be enforced by the lua vm itself
    /// (e.g. using mlua)
    pub fn set_memory_limit(&self, limit: usize) -> Result<usize, LuaError> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        lua.set_memory_limit(limit)
    }

    /// Execute a closure with the lua vm if it is valid
    pub fn with_lua<F, R>(&self, func: F) -> LuaResult<R>
    where
        F: FnOnce(&Lua) -> LuaResult<R>,
    {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        self.handle_error(func(lua))
    }

    /// Helper methods to handle errors correctly, dispatching mark_broken calls if theres
    /// a memory error etc.
    pub fn handle_error<T>(&self, resp: LuaResult<T>) -> LuaResult<T> {
        match resp {
            Ok(f) => Ok(f),
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
    }

    /// Loads/evaluates a script
    pub fn eval_script<R>(
        &self,
        path: &str,
    ) -> LuaResult<R> 
    where
        R: FromLuaMulti,
    {
        // Ensure create_thread wont error
        self.update_last_execution_time(std::time::Instant::now());
        self.handle_error(self.proxy_require.call(path))
    }

    /// Loads/evaluates a chunk of code into a function
    pub fn eval_chunk(
        &self,
        code: &str,
        name: Option<&str>,
        env: Option<LuaTable>,
    ) -> LuaResult<LuaFunction> {
        // Ensure create_thread wont error
        self.update_last_execution_time(std::time::Instant::now());
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };

        let chunk = match name {
            Some(n) => lua.load(code).set_name(n),
            None => lua.load(code),
        };
        let chunk = match env {
            Some(e) => chunk.set_environment(e),
            None => chunk.set_environment(self.global_table.clone()),
        };
        let chunk = chunk
            .set_compiler(self.compiler.clone())
            .set_mode(mluau::ChunkMode::Text)
            .try_cache();

        self.handle_error(chunk.into_function())
    }

    /// Helper method to call a function inside of the scheduler as a thread
    pub async fn call_in_scheduler<A, R>(
        &self,
        func: LuaFunction,
        args: A,
    ) -> LuaResult<R>
    where
        A: IntoLuaMulti,
        R: FromLuaMulti,
    {
        // Ensure create_thread wont error
        self.update_last_execution_time(std::time::Instant::now());
        let (th, args) = {
            let Some(ref lua) = *self.lua.borrow() else {
                return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
            };
            (self.handle_error(lua.create_thread(func))?, self.handle_error(args.into_lua_multi(lua))?)
        };

        // Update last_execution_time
        self.update_last_execution_time(std::time::Instant::now());

        let res = self.handle_error(self
            .scheduler
            .spawn_thread_and_wait(th, args)
            .await)?;

        {
            let Some(ref lua) = *self.lua.borrow() else {
                return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
            };

            let Some(res) = res else {
                return Ok(R::from_lua_multi(LuaMultiValue::with_capacity(0), lua)?)
            };

            let res = self.handle_error(res)?;
            self.handle_error(R::from_lua_multi(res, lua))
        }
    }

    pub fn from_value<T: for<'de> serde::Deserialize<'de>>(&self, value: LuaValue) -> LuaResult<T> {
        let Some(ref lua) = *self.lua.borrow() else {
            return Err(LuaError::RuntimeError("Lua VM is not valid".to_string()));
        };
        self.handle_error(lua.from_value(value))
    }

    /// Closes the lua vm and marks the runtime as broken
    ///
    /// This is similar to ``mark_broken`` but will not call any callbacks
    pub fn close(&self) -> Result<(), crate::Error> {
        self.broken.set(true); // Mark the runtime as broken if it is closed

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
        self.broken.set(true); // Mark the runtime as broken if it is closed

        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.lua.borrow().is_none()
    }
}

/// Creates a proxy global table that forwards reads to the global table if the key is in the global table
///
/// The resulting proxied global table includes
pub fn proxy_global(lua: &Lua) -> LuaResult<LuaTable> {
    // Setup the global table using a metatable
    //
    // SAFETY: This works because the global table will not change in the VM
    let global_mt = lua.create_table()?;
    let global_tab = lua.create_table()?;

    // Proxy reads to globals if key is in globals, otherwise to the table
    global_mt.set("__index", lua.globals())?;
    global_tab.set("_G", global_tab.clone())?;

    // Forward writes to the table itself
    /*global_mt.set(
        "__newindex",
        lua.create_function(
            move |_lua, (tab, key, value): (LuaTable, LuaValue, LuaValue)| {
                tab.raw_set(key, value)
            },
        )?,
    )?;*/

    lua.gc_collect()?;

    // Used in iterator
    let lua_global_pairs = Rc::new(
        lua.globals()
            .pairs()
            .collect::<LuaResult<Vec<(LuaValue, LuaValue)>>>()?,
    );

    // Provides iteration over first the users globals, then lua.globals()
    //
    // This is done using a Luau script to avoid borrowing issues
    global_mt.set(
        "__iter",
        lua.create_function(move |lua, globals: LuaTable| {
            let global_pairs = globals
                .pairs()
                .collect::<LuaResult<Vec<(LuaValue, LuaValue)>>>()?;

            let lua_global_pairs = lua_global_pairs.clone();

            let i = Cell::new(0);
            let iter = lua.create_function(move |_lua, ()| {
                let curr_i = i.get();

                if curr_i < global_pairs.len() {
                    let Some((key, value)) = global_pairs.get(curr_i).cloned() else {
                        return Ok((LuaValue::Nil, LuaValue::Nil));
                    };
                    i.set(curr_i + 1);
                    return Ok((key, value));
                }

                if curr_i < global_pairs.len() + lua_global_pairs.len() {
                    let Some((key, value)) =
                        lua_global_pairs.get(curr_i - global_pairs.len()).cloned()
                    else {
                        return Ok((LuaValue::Nil, LuaValue::Nil));
                    };
                    i.set(curr_i + 1);
                    return Ok((key, value));
                }

                Ok((LuaValue::Nil, LuaValue::Nil))
            })?;

            Ok(iter)
        })?,
    )?;

    global_mt.set(
        "__len",
        lua.create_function(move |lua, globals: LuaTable| {
            let globals_len = globals.raw_len();
            let len = lua.globals().raw_len();
            Ok(globals_len + len)
        })?,
    )?;

    // Block getmetatable
    global_mt.set("__metatable", false)?;

    global_tab.set_metatable(Some(global_mt))?;
    
    /*if tflags.contains(TFlags::READONLY_GLOBALS) {
        global_tab.set_safeenv(true);
    }*/

    Ok(global_tab)
}