
//! Module providing a wrapper for the native Julia module object.

use sys::*;
use error::{Result, Error};
use super::{Ref, Function, IntoSymbol};

wrap_ref! { pub struct Module(Ref); }

impl Module {
    /// Returns a global bound to the symbol `sym`.
    pub fn global<S: IntoSymbol>(&self, sym: S) -> Result<Ref> {
        let module = self.lock()?;
        let sym = sym.into_symbol()?;
        let sym = sym.lock()?;
        let raw = unsafe { jl_get_global(module, sym) };
        if raw.is_null() {
            Err(Error::NullPointer)
        } else {
            Ok(Ref::new(raw))
        }
    }

    /// Returns a function bound to the symbol `sym`.
    pub fn function<S: IntoSymbol>(&self, sym: S) -> Result<Function> {
        self.global(sym.into_symbol()?).map(Function)
    }

    /// Binds `value` to the symbol `sym` in this module.
    pub fn set<S: IntoSymbol>(&self, sym: S, value: &Ref) -> Result<()> {
        let module = self.lock()?;
        let sym = sym.into_symbol()?;
        let sym = sym.lock()?;
        let val = value.lock()?;
        unsafe {
            jl_set_global(module, sym, val);
        }
        Ok(())
    }

    /// Binds `value` to the symbol `sym` in this module as a constant.
    pub fn set_const<S: IntoSymbol>(&self, sym: S, value: &Ref) -> Result<()> {
        let module = self.lock()?;
        let sym = sym.into_symbol()?;
        let sym = sym.lock()?;
        let val = value.lock()?;
        unsafe {
            jl_set_const(module, sym, val);
        }
        Ok(())
    }
}
