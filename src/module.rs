
use sys::*;
use error::{Result, Error};
use value::{JlValue, Value};
use function::Function;
use sym::IntoSymbol;

jlvalues! {
    pub struct Module(jl_module_t);
}

impl Module {
    pub fn global<S: IntoSymbol>(&self, sym: S) -> Result<Value> {
        let module = self.lock()?;
        let sym = sym.into_symbol()?;
        let sym = sym.into_inner()?;
        let raw = unsafe { jl_call!(jl_get_global, module, sym) };
        Value::new(raw).map_err(|_| Error::UndefVar)
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
            jl_call!(jl_set_global, module, sym, val);
        }
        Ok(())
    }

    pub fn set_const<S: IntoSymbol>(&self, sym: S, value: &Value) -> Result<()> {
        let module = self.lock()?;
        let sym = sym.into_symbol()?;
        let sym = sym.into_inner()?;
        let val = value.lock()?;
        unsafe {
            jl_call!(jl_set_const, module, sym, val);
        }
        Ok(())
    }
}
