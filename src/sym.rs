
use sys::*;
use error::{Result, Error};
use string::IntoCString;
use value::JlValue;

pub trait IntoSymbol {
    fn into_symbol(self) -> Result<Symbol>;
}

jlvalues! {
    pub struct Symbol(jl_sym_t);
}

impl Symbol {
    pub fn with_name<S: IntoCString>(name: S) -> Result<Symbol> {
        let name = name.into_cstring();
        let raw = unsafe { jl_symbol(name.as_ptr()) };
        Symbol::new(raw).map_err(|_| Error::InvalidSymbol)
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
