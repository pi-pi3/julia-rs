
//! Module providing wrappers for the native Julia type-types.

use sys::*;
use error::Result;
use api::{Value, JlValue, Array};

jlvalues! {
    pub struct Datatype(jl_datatype_t);
    pub struct Union(jl_uniontype_t);
    pub struct UnionAll(jl_unionall_t);
}

impl Datatype {
    /// Creates a new Julia struct of this type. 
    pub fn new_struct<I>(&self, params: I) -> Result<Value>
    where
        I: IntoIterator<Item = Value>,
    {
        let mut paramv = vec![];
        for p in params {
            paramv.push(p.lock()?);
        }
        let nparam = paramv.len();
        let paramv = paramv.as_mut_ptr();

        let dt = self.lock()?;
        let value = unsafe { jl_new_structv(dt, paramv, nparam as u32) };
        jl_catch!();
        Value::new(value)
    }

    /// Creates a new Julia array of this type. 
    pub fn new_array<I>(&self, params: I) -> Result<Array>
    where
        I: IntoIterator<Item = Value>,
    {
        let mut paramv = vec![];
        for p in params {
            paramv.push(p.lock()?);
        }

        let dt = self.lock()?;
        let array = unsafe { jl_alloc_array_1d(dt as *mut _, paramv.len()) };
        jl_catch!();

        for (i, p) in paramv.into_iter().enumerate() {
            unsafe {
                jl_arrayset(array, p, i);
            }
            jl_catch!();
        }

        Array::new(array)
    }
}
