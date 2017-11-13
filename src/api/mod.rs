
use std::io::Read;
use std::ptr;

use sys::*;
use error::{Result, Error};
use string::IntoCString;

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
pub mod types;

pub use self::value::{Value, JlValue};
pub use self::array::{Array, Svec};
pub use self::function::Function;
pub use self::sym::{Symbol, IntoSymbol};
pub use self::module::Module;
pub use self::datatype::Datatype;
pub use self::task::Task;
pub use self::exception::Exception;
pub use self::types::*;

pub struct Gc;

impl Gc {
    pub fn enable(&self, p: bool) -> Result<()> {
        unsafe {
            jl_gc_enable(p as i32);
        }
        jl_catch!();
        Ok(())
    }

    pub fn is_enabled(&self) -> bool {
        unsafe {
            jl_gc_is_enabled() != 0
        }
    }

    pub fn collect(&self, full: bool) -> Result<()> {
        unsafe {
            jl_gc_collect(full as i32);
        }
        jl_catch!();
        Ok(())
    }

    pub fn total_bytes(&self) -> isize {
        unsafe {
            jl_gc_total_bytes() as isize
        }
    }

    pub fn total_hrtime(&self) -> usize {
        unsafe {
            jl_gc_total_hrtime() as usize
        }
    }

    pub fn diff_total_bytes(&self) -> isize {
        unsafe {
            jl_gc_diff_total_bytes() as isize
        }
    }
}

pub struct Julia {
    main: Module,
    core: Module,
    base: Module,
    top: Module,
    status: i32,
    gc: Gc,
}

impl Julia {
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

    pub fn gc(&self) -> &Gc {
        &self.gc
    }

    pub fn is_initialized() -> bool {
        unsafe { jl_is_initialized() != 0 }
    }

    pub fn exit(mut self, status: i32) {
        self.status = status
    }

    pub fn main(&self) -> &Module {
        &self.main
    }

    pub fn core(&self) -> &Module {
        &self.core
    }

    pub fn base(&self) -> &Module {
        &self.base
    }

    pub fn top(&self) -> &Module {
        &self.top
    }

    pub fn load<R: Read>(r: &mut R) -> Result<Value> {
        let mut content = String::new();
        let len = r.read_to_string(&mut content)?;
        let content = content.into_cstring();
        let content = content.as_ptr();

        let raw = unsafe { jl_load_file_string(content, len, ptr::null::<i8>() as *mut _) };
        jl_catch!();
        Value::new(raw)
    }

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
