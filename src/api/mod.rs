
//! Main entry point to the Julia api.

use std::io::Read;
use std::ffi::CStr;

use sys::*;
use error::{Result, Error};
use version::Version;
use string::IntoCString;

/// This macro checks for exceptions that might have occurred in the sys::*
/// functions. Should be used after calling any jl_* function that might throw
/// an exception.
#[macro_export]
macro_rules! jl_catch {
    () => {
        jl_catch!(|ex| { ex });
    };
    (|$ex:ident| $body:expr) => {
        jl_catch!(|$ex -> $crate::error::Error::UnhandledException| $crate::error::Error::UnhandledException($body));
    };
    (|$ex:ident -> $t:ty| $body:expr) => {
        #[allow(unused_variables)] // this shouldn't be necessary
        {
            if let Some($ex) = $crate::api::Exception::catch() {
                return Err($body);
            }
        }
    }
}

#[macro_use]
pub mod value;
#[macro_use]
pub mod array;
pub mod function;
pub mod sym;
pub mod module;
pub mod datatype;
pub mod task;
pub mod exception;
pub mod primitive;

pub use self::value::{Value, JlValue};
pub use self::array::{Array, Svec};
pub use self::function::Function;
pub use self::sym::{Symbol, IntoSymbol};
pub use self::module::Module;
pub use self::datatype::Datatype;
pub use self::task::Task;
pub use self::exception::Exception;
pub use self::primitive::*;

/// Blank struct for controlling the Julia garbage collector.
pub struct Gc;

impl Gc {
    /// Enable or disable the garbage collector.
    pub fn enable(&self, p: bool) -> Result<()> {
        unsafe {
            jl_gc_enable(p as i32);
        }
        jl_catch!();
        Ok(())
    }

    /// Check to see if gc is enabled.
    pub fn is_enabled(&self) -> bool {
        unsafe { jl_gc_is_enabled() != 0 }
    }

    /// Collect immediately. Set full to true if a full garbage collection
    /// should be issued
    pub fn collect(&self, full: bool) -> Result<()> {
        unsafe {
            jl_gc_collect(full as i32);
        }
        jl_catch!();
        Ok(())
    }

    /// Total bytes in use by the gc.
    pub fn total_bytes(&self) -> isize {
        unsafe { jl_gc_total_bytes() as isize }
    }

    pub fn total_hrtime(&self) -> usize {
        unsafe { jl_gc_total_hrtime() as usize }
    }

    pub fn diff_total_bytes(&self) -> isize {
        unsafe { jl_gc_diff_total_bytes() as isize }
    }
}

/// Struct for controlling the Julia runtime.
pub struct Julia {
    main: Module,
    core: Module,
    base: Module,
    top: Module,
    status: i32,
    gc: Gc,
}

impl Julia {
    /// Initialize the Julia runtime.
    ///
    /// ## Errors
    ///
    /// Returns Error::JuliaInitialized if Julia is already initialized.
    pub fn new() -> Result<Julia> {
        if Julia::is_initialized() {
            return Err(Error::JuliaInitialized);
        }

        unsafe {
            jl_init();
        }
        jl_catch!();

        let main = unsafe { Module::new_unchecked(jl_main_module) };
        let core = unsafe { Module::new_unchecked(jl_core_module) };
        let base = unsafe { Module::new_unchecked(jl_base_module) };
        let top = unsafe { Module::new_unchecked(jl_top_module) };

        Ok(Julia {
            main: main,
            core: core,
            base: base,
            top: top,
            status: 0,
            gc: Gc,
        })
    }

    /// Returns the version of currently running Julia runtime.
    pub fn version(&self) -> Version {
        unsafe {
            let major = jl_ver_major() as u32;
            let minor = jl_ver_minor() as u32;
            let patch = jl_ver_patch() as u32;
            let release = jl_ver_is_release() != 0;
            let branch = jl_git_branch();
            let commit = jl_git_commit();
            let mut branch = CStr::from_ptr(branch).to_str().ok();
            let commit = CStr::from_ptr(commit).to_str().ok();

            if branch == Some("(no branch)") {
                branch = None;
            }

            Version {
                name: "julia",
                major: major,
                minor: minor,
                patch: patch,
                release: release,
                branch: branch,
                commit: commit,
            }
        }
    }

    /// Returns a reference to the garbage collector.
    pub fn gc(&self) -> &Gc {
        &self.gc
    }

    /// Checks if Julia was already initialized in the current thread.
    pub fn is_initialized() -> bool {
        unsafe { jl_is_initialized() != 0 }
    }

    /// Sets own status to status and consumes Julia, causing the value to be
    /// dropped.
    pub fn exit(mut self, status: i32) {
        self.status = status
    }

    /// Returns a handle to the main module.
    pub fn main(&self) -> &Module {
        &self.main
    }

    /// Returns a handle to the core module.
    pub fn core(&self) -> &Module {
        &self.core
    }

    /// Returns a handle to the base module.
    pub fn base(&self) -> &Module {
        &self.base
    }

    /// Returns a handle to the top module.
    pub fn top(&self) -> &Module {
        &self.top
    }

    /// Loads a Julia script from any Read without evaluating it.
    pub fn load<R: Read, S: IntoCString>(&mut self, r: &mut R, name: Option<S>) -> Result<Value> {
        let mut content = String::new();
        let len = r.read_to_string(&mut content)?;
        let content = content.into_cstring();
        let content = content.as_ptr();

        let name = name.map(|s| s.into_cstring()).unwrap_or_else(
            || "string".into_cstring(),
        );
        let name = name.as_ptr();

        //let raw = unsafe { jl_load_file_string(content, len, ptr::null::<i8>() as *mut _) };
        let raw = unsafe { jl_load_file_string(content, len, name as *mut _) };
        jl_catch!();
        Value::new(raw)
    }

    /// Parses and evaluates string.
    pub fn eval_string<S: IntoCString>(&mut self, string: S) -> Result<Value> {
        let string = string.into_cstring();
        let string = string.as_ptr();

        let ret = unsafe { jl_eval_string(string) };
        jl_catch!();
        Value::new(ret).map_err(|_| Error::EvalError)
    }
}

impl Drop for Julia {
    fn drop(&mut self) {
        unsafe {
            jl_atexit_hook(self.status);
        }
    }
}
