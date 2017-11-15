
//! Safe and idiomatic [Julia](https://julialang.org) bindings for
//! [Rust](https://rust-lang.org).
//! [#JuliaLang](https://twitter.com/search?q=%23JuliaLang)
//! [#RustLang](https://twitter.com/search?q=%23RustLang)
//!
//! Uses nightly Rust for compilation, rustfmt with default settings for
//! formatting, clippy for checking and resolving lints.
//!
//! julia-sys are the raw ffi bindings for Julia generated with
//! [bindgen](https://crates.io/crates/bindgen).
//!
//! # Example
//!
//! An example of using Rust to interface with Julia.
//!
//! ```
//! fn main() {
//!     use julia::api::{Julia, Value};
//!
//!     let mut jl = Julia::new().unwrap();
//!     jl.eval_string("println(\"Hello, Julia!\")").unwrap();
//!     // Hello, Julia!
//!
//!     let sqrt = jl.base().function("sqrt").unwrap();
//!
//!     let boxed_x = Value::from(1337.0);
//!     let boxed_sqrt_x = sqrt.call1(&boxed_x).unwrap();
//!
//!     let sqrt_x = f64::try_from(boxed_sqrt_x).unwrap();
//!     println!("{}", sqrt_x);
//!     // 36.565010597564445
//! }
//! ```

#![feature(try_from)]
#![feature(unique)]
#![feature(concat_idents)]

#![allow(unknown_lints)]
#![allow(not_unsafe_ptr_arg_deref)]
#![allow(match_same_arms)]
#![allow(doc_markdown)]

extern crate libc;
extern crate smallvec;
extern crate julia_sys;

pub mod sys;
#[macro_use]
pub mod ext;
pub mod error;
#[macro_use]
pub mod string;
pub mod version;

#[macro_use]
pub mod api;

#[cfg(test)]
mod tests {
    use super::api::Julia;

    #[test]
    fn sanity() {
        let _jl = Julia::new();
    }
}
