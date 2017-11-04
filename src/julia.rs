
use sys::*;
use value::{JlValue, Value};
use function::Function;
use sym::Symbol;
use module::Module;
use error::{Result, Error};
use string::AsCString;

pub struct Julia {
    main: Module,
    internal_main: Module,
    core: Module,
    base: Module,
    top: Module,
}

impl Julia {
    pub fn new() -> Julia {
        unsafe {
            jl_init();
        }

        let main = unsafe { Module::new_unchecked(jl_main_module) };
        let internal_main = unsafe { Module::new_unchecked(jl_internal_main_module) };
        let core = unsafe { Module::new_unchecked(jl_core_module) };
        let base = unsafe { Module::new_unchecked(jl_base_module) };
        let top = unsafe { Module::new_unchecked(jl_top_module) };

        Julia {
            main: main,
            internal_main: internal_main,
            core: core,
            base: base,
            top: top,
        }
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

    pub fn eval_string<S: AsCString>(&mut self, string: S) -> Result<Value> {
        let string = string.as_cstring();

        let ret = unsafe { jl_eval_string(string.as_ptr()) };
        Value::new(ret).map_err(|_| Error::EvalError)
    }

    pub fn get_global<S: AsCString>(&self, module: &Module, sym: S) -> Result<Value> {
        let module = module.lock()?;
        let sym = Symbol::with_name(sym.as_cstring())?;
        let sym = sym.into_inner()?;
        let raw = unsafe { jl_get_global(module, sym) };
        Value::new(raw).map_err(|_| Error::UndefVar)
    }

    pub fn set_global<S: AsCString>(&self, module: &Module, sym: S, value: &Value) -> Result<()> {
        let module = module.lock()?;
        let sym = Symbol::with_name(sym.as_cstring())?;
        let sym = sym.into_inner()?;
        let val = value.lock()?;
        unsafe {
            jl_set_global(module, sym, val);
        }
        Ok(())
    }

    pub fn set_const<S: AsCString>(&self, module: &Module, sym: S, value: &Value) -> Result<()> {
        let module = module.lock()?;
        let sym = Symbol::with_name(sym.as_cstring())?;
        let sym = sym.into_inner()?;
        let val = value.lock()?;
        unsafe {
            jl_set_const(module, sym, val);
        }
        Ok(())
    }

    pub fn get_function<S: AsCString>(&self, module: &Module, sym: S) -> Result<Function> {
        self.get_global(module, sym.as_cstring()).and_then(
            Function::from_value,
        )
    }
}

impl Drop for Julia {
    fn drop(&mut self) {
        unsafe {
            jl_atexit_hook(0);
        }
    }
}

impl Default for Julia {
    fn default() -> Julia {
        Julia::new()
    }
}
