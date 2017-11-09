
use smallvec::SmallVec;

use sys::*;
use error::{Result, Error};
use super::{JlValue, Value};

jlvalues! {
    pub struct Function(jl_function_t);
}

impl Function {
    pub fn call<'a, I>(&self, args: I) -> Result<Value>
    where
        I: IntoIterator<Item = &'a Value>,
    {
        let mut argv = SmallVec::<[*mut jl_value_t; 8]>::new();
        for arg in args {
            argv.push(arg.lock()?);
        }

        let ret = unsafe {
            jl_call(self.lock()?, argv.as_mut_ptr(), argv.len() as i32)
        };
        jl_catch!();
        Value::new(ret).map_err(|_| Error::CallError)
    }

    pub fn call0(&self) -> Result<Value> {
        let ret = unsafe {
            jl_call0(self.lock()?)
        };
        jl_catch!();
        Value::new(ret).map_err(|_| Error::CallError)
    }

    pub fn call1(&self, arg1: &Value) -> Result<Value> {
        let ret = unsafe {
            jl_call1(self.lock()?, arg1.lock()?)
        };
        jl_catch!();
        Value::new(ret).map_err(|_| Error::CallError)
    }

    pub fn call2(&self, arg1: &Value, arg2: &Value) -> Result<Value> {
        let ret = unsafe {
            jl_call2(self.lock()?, arg1.lock()?, arg2.lock()?)
        };
        jl_catch!();
        Value::new(ret).map_err(|_| Error::CallError)
    }

    pub fn call3(&self, arg1: &Value, arg2: &Value, arg3: &Value) -> Result<Value> {
        let ret = unsafe {
            jl_call3(self.lock()?, arg1.lock()?, arg2.lock()?, arg3.lock()?)
        };
        jl_catch!();
        Value::new(ret).map_err(|_| Error::CallError)
    }
}
