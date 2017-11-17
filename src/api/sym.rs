
//! Module providing a wrapper for the native Julia symbol.

use std::convert::TryFrom;

use sys::*;
use error::{Result, Error};
use string::{IntoCString, TryIntoString};
use api::Ref;

/// Trait implemented by every type which can be used to construct a Symbol.
pub trait IntoSymbol {
    fn into_symbol(self) -> Result<Symbol>;
}

wrap_ref! { pub struct Symbol(Ref); }

impl Symbol {
    /// Construct a new symbol with a name.
    pub fn with_name<S: IntoCString>(name: S) -> Result<Symbol> {
        let name = name.into_cstring();
        let raw = unsafe { jl_symbol(name.as_ptr()) };
        jl_catch!();
        let sym = Symbol(Ref::new(raw));
        Ok(sym)
    }

    // This never fails.
    /// Procedurally generates a new symbol.
    pub fn gensym() -> Symbol {
        let raw = unsafe { jl_gensym() };
        Symbol(Ref::new(raw))
    }

    // This never fails.
    /// Returns `symtab`, the root symbol.
    pub fn get_root() -> Symbol {
        let raw = unsafe { jl_get_root_symbol() };
        Symbol(Ref::new(raw))
    }
}

impl IntoSymbol for Symbol {
    fn into_symbol(self) -> Result<Symbol> {
        Ok(self)
    }
}

impl<S: IntoCString> IntoSymbol for S {
    fn into_symbol(self) -> Result<Symbol> {
        Symbol::with_name(self.into_cstring())
    }
}

impl<'a> TryFrom<&'a Symbol> for String {
    type Error = Error;
    fn try_from(sym: &Symbol) -> Result<String> {
        let ptr = unsafe { jl_symbol_name(sym.lock()?) };
        jl_catch!();
        Ok(ptr.try_into_string().unwrap())
    }
}
