
//! Module providing a wrapper for the native Julia function object.

use smallvec::SmallVec;

use sys::*;
use error::Result;
use api::Ref;

wrap_ref! { pub struct Function(Ref); }

impl Function {
    /// Call with a sequence of Ref-s.
    pub fn call<'a, I>(&self, args: I) -> Result<Ref>
    where
        I: IntoIterator<Item = &'a Ref>,
    {
        let mut argv = SmallVec::<[*mut jl_value_t; 8]>::new();
        for arg in args {
            argv.push(arg.lock()?);
        }

        let raw = self.lock()?;

        let ret =
            except! {
            try {
                unsafe { jl_call(raw, argv.as_mut_ptr(), argv.len() as i32) }
            } catch ex => {
                rethrow!(ex)
            }
        };
        Ok(Ref::new(ret))
    }

    /// Call with 0 Ref-s.
    pub fn call0(&self) -> Result<Ref> {
        let raw = self.lock()?;

        let ret =
            except! {
            try {
                unsafe { jl_call0(raw) }
            } catch ex => {
                rethrow!(ex)
            }
        };
        Ok(Ref::new(ret))
    }

    /// Call with 1 Ref.
    pub fn call1(&self, arg1: &Ref) -> Result<Ref> {
        let raw = self.lock()?;

        let ret =
            except! {
            try {
                unsafe { jl_call1(raw, arg1.lock()?) }
            } catch ex => {
                rethrow!(ex)
            }
        };
        Ok(Ref::new(ret))
    }

    /// Call with 2 Ref-s.
    pub fn call2(&self, arg1: &Ref, arg2: &Ref) -> Result<Ref> {
        let raw = self.lock()?;

        let ret =
            except! {
            try {
                unsafe { jl_call2(raw, arg1.lock()?, arg2.lock()?) }
            } catch ex => {
                rethrow!(ex)
            }
        };
        Ok(Ref::new(ret))
    }

    /// Call with 3 Ref-s.
    pub fn call3(&self, arg1: &Ref, arg2: &Ref, arg3: &Ref) -> Result<Ref> {
        let raw = self.lock()?;

        let ret =
            except! {
            try {
                unsafe { jl_call3(raw, arg1.lock()?, arg2.lock()?, arg3.lock()?) }
            } catch ex => {
                rethrow!(ex)
            }
        };
        Ok(Ref::new(ret))
    }
}
