
//! Main entry point to the Julia api.

use std::io::Read;
use std::ffi::CStr;

use sys::*;
use error::Result;
use version::Version;
use string::IntoCString;

/// Used for ffi calls that might throw an exception. If an exception occurs, it
/// can be handled gracefully or ignored.
///
/// # Examples
///
/// ```
/// except! {
///     let result = try {
///         might_throw();
///         Ok(())
///     } catch ex => {
///         Err(ex)
///     };
/// }
/// ```
///
/// ```
/// except! {
///     let x = try {
///         might_throw();
///     } catch Exception::Bounds(ex) => {
///         eprintln!("Out of bounds: {:?}", ex);
///         return;
///     } catch Exception::Error(ex) => {
///         eprintln!("Generic error: {:?}", ex);
///         return;
///     } finally {
///         println!("Done");
///         Ok(())
///     };
/// }
/// ```
#[macro_export]
macro_rules! except {
    {
        try $try:block
    } => {
        {
            let try_val = $try;
            if let Some(ex) = $crate::api::Exception::catch() {
                panic!("unhandled exception\n > {:?}\n >> {}", ex, ex.description())
            }
            try_val
        }
    };
    {
        try $try:block
        finally $finally:block
    } => {
        {
            $try;
            if let Some(ex) = $crate::api::Exception::catch() {
                panic!("unhandled exception\n > {:?}\n >> {}", ex, ex.description())
            }
            $finally
        }
    };
    {
        try $try:block
        $( catch $ex:pat => $catch:block )*
    } => {
        {
            use std::error::Error as ErrorTrait;
            let try_val = $try;
            #[allow(unreachable_patterns)]
            match $crate::api::Exception::catch() {
                None => try_val,
                $(
                    Some($ex) => $catch,
                )*
                Some(ex) => panic!("unhandled exception\n > {:?}\n >> {}", ex, ex.description()),
            }
        }
    };
    {
        try $try:block
        $( catch $ex:pat => $catch:block )*
        finally $finally:block
    } => {
        {
            use std::error::Error as ErrorTrait;

            struct Finally;

            impl Drop for Finally {
                fn drop(&mut self) {
                    $finally;
                }
            }

            let _finally = Finally;

            $try;
            #[allow(unreachable_patterns)]
            match $crate::api::Exception::catch() {
                None => (),
                $(
                    Some($ex) => { $catch; },
                )*
                Some(ex) => panic!("unhandled exception\n > {:?}\n >> {}", ex, ex.description()),
            }

            // It's alright to forget `finally`, because it's zero-sized.
            ::std::mem::forget(_finally);

            $finally
        }
    };
}

/// Used to return a caught Julia exception as a Rust error.
#[macro_export]
macro_rules! rethrow {
    ( $ex:expr ) => { return Err($crate::error::Error::UnhandledException($ex)); }
}

#[macro_use]
pub mod reference;
#[macro_use]
pub mod array;
pub mod function;
pub mod sym;
pub mod expr;
pub mod module;
pub mod datatype;
pub mod task;
pub mod exception;
pub mod primitive;

pub use self::reference::{Ref, ToJulia, FromJulia};
pub use self::array::{Array, ByteArray, Svec};
pub use self::function::Function;
pub use self::sym::{Symbol, IntoSymbol};
pub use self::expr::Expr;
pub use self::module::Module;
pub use self::datatype::Datatype;
pub use self::task::Task;
pub use self::exception::Exception;
pub use self::primitive::*;

/// Blank struct for controlling the Julia garbage collector.
pub struct Gc;

impl Gc {
    /// Enable or disable the garbage collector.
    pub fn enable(&mut self, p: bool) {
        unsafe {
            jl_gc_enable(p as i32);
        }
    }

    /// Check to see if gc is enabled.
    pub fn is_enabled(&self) -> bool {
        unsafe { jl_gc_is_enabled() != 0 }
    }

    /// Collect immediately. Set full to true if a full garbage collection
    /// should be issued
    pub fn collect(&mut self, full: bool) {
        unsafe {
            jl_gc_collect(full as i32);
        }
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
    at_exit: Option<i32>,
    gc: Gc,
}

impl Julia {
    /// Assume that Julia was already initialized somewhere else and return a
    /// handle.
    ///
    /// This function is unsafe, because if any Julia operation is called, it
    /// will likely segfault. Also, the 4 jl_* modules might be null.
    ///
    /// # Panics
    ///
    /// Panics if the Julia runtime was not previously initialized.
    pub unsafe fn new_unchecked() -> Julia {
        if !Julia::is_initialized() {
            panic!("Julia is not initialized");
        }

        let main = Module(Ref::new(jl_main_module));
        let core = Module(Ref::new(jl_core_module));
        let base = Module(Ref::new(jl_base_module));
        let top = Module(Ref::new(jl_top_module));

        Julia {
            main: main,
            core: core,
            base: base,
            top: top,
            at_exit: None,
            gc: Gc,
        }
    }

    /// Initialize the Julia runtime.
    ///
    /// # Panics
    ///
    /// Panics if Julia was already initialized somewhere else. Use
    /// `Julia::new_unchecked` if you want to receive a handle to Julia anyway.
    pub fn new() -> Julia {
        if Julia::is_initialized() {
            panic!("Julia was already initialized");
        }

        unsafe {
            jl_init();
        }

        let mut jl = unsafe { Julia::new_unchecked() };
        jl.at_exit = Some(0);
        jl
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

    /// Returns a mutable reference to the garbage collector.
    pub fn gc_mut(&mut self) -> &mut Gc {
        &mut self.gc
    }

    /// Checks if Julia was already initialized in the current thread.
    pub fn is_initialized() -> bool {
        unsafe { jl_is_initialized() != 0 }
    }

    /// Sets status to at_exit and consumes Julia, causing the value to be
    /// dropped.
    pub fn exit(mut self, at_exit: i32) {
        self.at_exit(Some(at_exit))
    }

    /// Sets status.
    pub fn at_exit(&mut self, at_exit: Option<i32>) {
        self.at_exit = at_exit;
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
    pub fn load<R: Read, S: IntoCString>(&mut self, r: &mut R, name: Option<S>) -> Result<Ref> {
        let mut content = String::new();
        let len = r.read_to_string(&mut content)?;
        let content = content.into_cstring();
        let content = content.as_ptr();

        let name = name.map(|s| s.into_cstring()).unwrap_or_else(
            || "unnamed.jl".into_cstring(),
        );
        let name = name.as_ptr();

        let raw =
            except! {
            try {
                unsafe {
                    jl_load_file_string(content as *mut _, len, name as *mut _)
                }
            } catch ex => {
                rethrow!(ex)
            }
        };
        Ok(Ref::new(raw))
    }

    /// Parses and evaluates string.
    pub fn eval_string<S: IntoCString>(&mut self, string: S) -> Result<Ref> {
        let string = string.into_cstring();
        let string = string.as_ptr();

        let ret =
            except! {
            try {
                unsafe {
                    jl_eval_string(string)
                }
            } catch ex => {
                rethrow!(ex)
            }
        };
        Ok(Ref::new(ret))
    }
}

impl Drop for Julia {
    fn drop(&mut self) {
        self.at_exit.map(|s| unsafe { jl_atexit_hook(s) });
    }
}
