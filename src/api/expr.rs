
//! Module containing traits, types and macros for interfacing with Julia
//! values.

use sys::*;
use error::Result;
use string::IntoCString;
use api::Ref;

wrap_ref! { pub struct Expr(Ref); }

impl Expr {
    /// Parse a string without evaluating it.
    pub fn with_string(string: &str) -> Result<Expr> {
        let len = string.len();
        let string = string.into_cstring();
        let string = string.as_ptr();

        let raw = except! {
            try {
                unsafe { jl_parse_string(string, len, 0, 0) }
            } catch ex => {
                rethrow!(ex)
            }
        };

        Ok(Expr(Ref::new(raw)))
    }

    /// Evaluate expression.
    pub fn expand(&self) -> Result<Ref> {
        let raw = self.lock()?;
        let raw = except! {
            try {
                unsafe { jl_expand(raw) }
            } catch ex => {
                rethrow!(ex)
            }
        };
        Ok(Ref::new(raw))
    }
}
