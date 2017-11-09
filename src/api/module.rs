
use sys::*;
use error::Result;
use super::{Value, JlValue, Function, IntoSymbol};

jlvalues! {
    pub struct Module(jl_module_t);
}

impl Module {
    pub fn global<S: IntoSymbol>(&self, sym: S) -> Result<Value> {
        let module = self.lock()?;
        let sym = sym.into_symbol()?;
        let sym = sym.into_inner()?;
        let raw = unsafe {
            jl_get_global(module, sym)
        };
        jl_catch!();
        Value::new(raw)
    }

    pub fn function<S: IntoSymbol>(&self, sym: S) -> Result<Function> {
        self.global(sym.into_symbol()?).and_then(
            Function::from_value,
        )
    }

    pub fn set<S: IntoSymbol>(&self, sym: S, value: &Value) -> Result<()> {
        let module = self.lock()?;
        let sym = sym.into_symbol()?;
        let sym = sym.into_inner()?;
        let val = value.lock()?;
        unsafe {
            jl_set_global(module, sym, val);
        }
        jl_catch!();
        Ok(())
    }

    pub fn set_const<S: IntoSymbol>(&self, sym: S, value: &Value) -> Result<()> {
        let module = self.lock()?;
        let sym = sym.into_symbol()?;
        let sym = sym.into_inner()?;
        let val = value.lock()?;
        unsafe {
            jl_set_const(module, sym, val);
        }
        jl_catch!();
        Ok(())
    }
}
