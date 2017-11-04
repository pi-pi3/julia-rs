
use sys::*;
use error::{Result, Error};
use string::AsCString;
use value::JlValue;

jlvalues! {
    pub struct Symbol(jl_sym_t);
}

impl Symbol {
    pub fn with_name<S: AsCString>(name: S) -> Result<Symbol> {
        let raw = unsafe { jl_symbol(name.as_cstring().as_ptr()) };
        Symbol::new(raw).map_err(|_| Error::InvalidSymbol)
    }
}
