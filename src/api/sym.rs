
//! Module providing a wrapper for the native Julia symbol.

use std::convert::TryFrom;
use std::ffi::CStr;

use sys::*;
use error::{Result, Error};
use string::IntoCString;
use super::JlValue;

/// Trait implemented by every type which can be used to construct a Symbol.
pub trait IntoSymbol {
    fn into_symbol(self) -> Result<Symbol>;
}

jlvalues! {
    pub struct Symbol(jl_sym_t);
}

impl Symbol {
    /// Construct a new symbol with a name.
    pub fn with_name<S: IntoCString>(name: S) -> Result<Symbol> {
        let name = name.into_cstring();
        let raw = unsafe { jl_symbol(name.as_ptr()) };
        Symbol::new(raw).map_err(|_| Error::InvalidSymbol)
    }

    // This never fails.
    /// Procedurally generates a new symbol.
    pub fn gensym() -> Symbol {
        unsafe {
            let raw = jl_gensym();
            Symbol::new_unchecked(raw)
        }
    }

    // This never fails.
    /// Returns `symtab`, the root symbol.
    pub fn get_root() -> Symbol {
        unsafe {
            let raw = jl_get_root_symbol();
            Symbol::new_unchecked(raw)
        }
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
        let raw = unsafe { jl_symbol_name(sym.lock()?) };
        jl_catch!();
        let cstr = unsafe { CStr::from_ptr(raw) };
        let cstring = cstr.to_owned();
        cstring.into_string().map_err(From::from)
    }
}
