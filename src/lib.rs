
#![feature(try_from)]
#![feature(unique)]
#![feature(concat_idents)]

#![allow(unknown_lints)]
#![allow(not_unsafe_ptr_arg_deref)]
#![allow(match_same_arms)]

extern crate libc;
extern crate smallvec;
extern crate julia_sys;

pub mod sys;
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
