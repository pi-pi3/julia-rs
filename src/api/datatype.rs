
use sys::*;
use error::Result;
use api::{Value, JlValue};

jlvalues! {
    pub struct Datatype(jl_datatype_t);
}

impl Datatype {
    pub fn construct<I>(&self, params: I) -> Result<Value>
    where
        I: IntoIterator<Item=Value> {
        let dt = self.lock()?;
        let mut paramv = vec![];
        for p in params {
            paramv.push(p.lock()?);
        }
        let nparam = paramv.len();
        let paramv = paramv.as_mut_ptr();

        let value = unsafe { jl_new_struct(dt, paramv, nparam) };
        jl_catch!();
        Value::new(value)
    }
}
