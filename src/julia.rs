
use std::path::Path;
use std::os::unix::ffi::OsStrExt;

use sys::*;
use value::{JlValue, Value};
use function::Function;
use sym::Symbol;
use module::Module;
use error::{Result, Error};
use string::AsCString;

#[macro_export]
macro_rules! jl_call {
    ($fun:path) => {
        jl_call!($fun,);
    };
    ($fun:path, $( $arg:expr ),*) => {
        {
            let ret = $fun( $( $arg ),* );
            let ex = $crate::exception::Exception::catch();
            if let Some(ex) = ex {
                return Err($crate::error::Error::UnhandledException(ex));
            }
            ret
        }
    }
}

pub struct Julia {
    main: Module,
    internal_main: Module,
    core: Module,
    base: Module,
    top: Module,
}

impl Julia {
    pub fn new() -> Result<Julia> {
        if Julia::is_initialized() {
            return Err(Error::JuliaInitialized);
        }

        unsafe {
            jl_call!(jl_init);
        }

        let main = unsafe { Module::new_unchecked(jl_main_module) };
        let internal_main = unsafe { Module::new_unchecked(jl_internal_main_module) };
        let core = unsafe { Module::new_unchecked(jl_core_module) };
        let base = unsafe { Module::new_unchecked(jl_base_module) };
        let top = unsafe { Module::new_unchecked(jl_top_module) };

        Ok(Julia {
            main: main,
            internal_main: internal_main,
            core: core,
            base: base,
            top: top,
        })
    }

    pub fn is_initialized() -> bool {
        unsafe {
            jl_is_initialized() != 0
        }
    }

    pub fn exit(&self, status: i32) -> ! {
        unsafe {
            jl_exit(status);
        }
        unreachable!()
    }

    pub fn main(&self) -> &Module {
        &self.main
    }

    pub fn internal_main(&self) -> &Module {
        &self.internal_main
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

    pub fn get_global<S: AsCString>(&self, module: &Module, sym: S) -> Result<Value> {
        let module = module.lock()?;
        let sym = Symbol::with_name(sym.as_cstring())?;
        let sym = sym.into_inner()?;
        let raw = unsafe { jl_call!(jl_get_global, module, sym) };
        Value::new(raw).map_err(|_| Error::UndefVar)
    }

    pub fn set_global<S: AsCString>(&self, module: &Module, sym: S, value: &Value) -> Result<()> {
        let module = module.lock()?;
        let sym = Symbol::with_name(sym.as_cstring())?;
        let sym = sym.into_inner()?;
        let val = value.lock()?;
        unsafe {
            jl_call!(jl_set_global, module, sym, val);
        }
        Ok(())
    }

    pub fn set_const<S: AsCString>(&self, module: &Module, sym: S, value: &Value) -> Result<()> {
        let module = module.lock()?;
        let sym = Symbol::with_name(sym.as_cstring())?;
        let sym = sym.into_inner()?;
        let val = value.lock()?;
        unsafe {
            jl_call!(jl_set_const, module, sym, val);
        }
        Ok(())
    }

    pub fn get_function<S: AsCString>(&self, module: &Module, sym: S) -> Result<Function> {
        self.get_global(module, sym.as_cstring()).and_then(
            Function::from_value,
        )
    }

    // TODO: AsCString
    pub fn parse_input_line<P: AsRef<Path>>(string: &str, filename: P) -> Result<Value> {
        let len = string.len();
        let string = string.as_cstring();
        let string = string.as_ptr();

        // TODO: this works only on windows
        // Also, bad hack
        // Also, I'm not sure if it works at all
        let filename = filename.as_ref().as_os_str().as_bytes();
        let filename_len = filename.len();
        let filename = filename.as_ptr() as *mut _;

        let raw = unsafe { jl_call!(jl_parse_input_line, string, len, filename, filename_len) };

        Value::new(raw)
    }

    pub fn parse_string(string: &str) -> Result<Value> {
        let len = string.len();
        let string = string.as_cstring();
        let string = string.as_ptr();

        let raw = unsafe { jl_call!(jl_parse_string, string, len, 0, 0) };

        Value::new(raw)
    }

    pub fn parse_depth_warn(warn: usize) -> Result<()> {
        unsafe {
            jl_call!(jl_parse_depwarn, warn as i32);
        }
        Ok(())
    }

    pub fn load_file_string<P: AsRef<Path>>(string: &str, filename: P) -> Result<Value> {
        let len = string.len();
        let string = string.as_cstring();
        let string = string.as_ptr();

        // TODO: this works only on windows
        // Also, bad hack
        // Also, I'm not sure if it works at all
        let filename = filename.as_ref().as_os_str().as_bytes().as_ptr() as *mut _;

        let raw = unsafe { jl_call!(jl_load_file_string, string, len, filename) };

        Value::new(raw)
    }

    pub fn eval_string<S: AsCString>(&mut self, string: S) -> Result<Value> {
        let string = string.as_cstring();
        let string = string.as_ptr();

        let ret = unsafe { jl_call!(jl_eval_string, string) };
        Value::new(ret).map_err(|_| Error::EvalError)
    }
}

impl Drop for Julia {
    fn drop(&mut self) {
        unsafe {
            jl_atexit_hook(0);
        }
    }
}
