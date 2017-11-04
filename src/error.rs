
use std::result;
use std::ffi::FromBytesWithNulError;
use std::sync::PoisonError;
use std::rc::Rc;
use std::char::CharTryFromError;
use std::fmt;

use smallvec::SmallVec;

use sys::*;
use value::{Value, JlValue};
use sym::Symbol;
use datatype::Datatype;
use string::AsCString;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Hash)]
pub enum Error {
    CStrError,
    InvalidUnbox,
    NotAFunction,
    CallError,
    EvalError,
    NullValue,
    PoisonError,
    ResourceInUse,
    UTF8Error,
    InvalidSymbol,
    UndefVar,
}

impl From<FromBytesWithNulError> for Error {
    fn from(_: FromBytesWithNulError) -> Error {
        Error::CStrError
    }
}

impl From<CharTryFromError> for Error {
    fn from(_: CharTryFromError) -> Error {
        Error::UTF8Error
    }
}

impl<G> From<PoisonError<G>> for Error {
    fn from(_: PoisonError<G>) -> Error {
        Error::PoisonError
    }
}

impl<T> From<Rc<T>> for Error {
    fn from(_: Rc<T>) -> Error {
        Error::ResourceInUse
    }
}

pub fn error<S: AsCString>(string: S) {
    let string = string.as_cstring().as_ptr();
    unsafe {
        jl_error(string);
    }
}

pub fn error_format(args: fmt::Arguments) {
    error(fmt::format(args).as_cstring());
}

pub fn exception<S: AsCString>(ty: &Datatype, string: S) -> Result<()> {
    let ty = ty.lock()?;
    let string = string.as_cstring().as_ptr();
    unsafe {
        jl_exceptionf(ty, string);
    }
    Ok(())
}

pub fn exception_format(ty: &Datatype, args: fmt::Arguments) -> Result<()> {
    exception(ty, fmt::format(args).as_cstring())
}

pub fn too_few_args<S: AsCString>(fname: S, min: usize) {
    let fname = fname.as_cstring().as_ptr();
    unsafe {
        jl_too_few_args(fname, min as i32);
    }
}

pub fn too_many_args<S: AsCString>(fname: S, max: usize) {
    let fname = fname.as_cstring().as_ptr();
    unsafe {
        jl_too_many_args(fname, max as i32);
    }
}

pub fn type_error<S: AsCString>(fname: S, expected: &Value, got: &Value) -> Result<()> {
    let fname = fname.as_cstring().as_ptr();
    let expected = expected.lock()?;
    let got = got.lock()?;
    unsafe {
        jl_type_error(fname, expected, got);
    }
    Ok(())
}

pub fn type_error_rt<S: AsCString>(fname: S, context: S, ty: &Value, got: &Value) -> Result<()> {
    let fname = fname.as_cstring().as_ptr();
    let context = context.as_cstring().as_ptr();
    let ty = ty.lock()?;
    let got = got.lock()?;
    unsafe {
        jl_type_error_rt(fname, context, ty, got);
    }
    Ok(())
}

pub fn undefined_var_error(var: &Symbol) -> Result<()> {
    let var = var.lock()?;
    unsafe {
        jl_undefined_var_error(var);
    }
    Ok(())
}

pub fn bounds_error(v: &Value, t: &Value) -> Result<()> {
    let v = v.lock()?;
    let t = t.lock()?;
    unsafe {
        jl_bounds_error(v, t);
    }
    Ok(())
}

pub fn bounds_error_v(v: &Value, idxs: &[Value]) -> Result<()> {
    let v = v.lock()?;
    let mut indices = SmallVec::<[*mut jl_value_t; 8]>::new();
    for i in idxs {
        indices.push(i.lock()?)
    }
    let nidxs = indices.len();
    let idxs = indices.as_mut_ptr();
    unsafe {
        jl_bounds_error_v(v, idxs, nidxs);
    }
    Ok(())
}

pub fn bounds_error_int(v: &Value, i: usize) -> Result<()> {
    let v = v.lock()?;
    unsafe {
        jl_bounds_error_int(v, i);
    }
    Ok(())
}

pub fn bounds_error_tuple_int(v: &[Value], i: usize) -> Result<()> {
    let mut vs = SmallVec::<[*mut jl_value_t; 8]>::new();
    for vi in v {
        vs.push(vi.lock()?);
    }
    let nv = vs.len();
    let v = vs.as_mut_ptr();
    unsafe {
        jl_bounds_error_tuple_int(v, nv, i);
    }
    Ok(())
}

// TODO
/*
pub fn bounds_error_unboxed_int(void *v, vt: &Value, i: usize) -> Result<()> {
    let vt = vt.lock()?;
    unsafe {
        jl_bounds_error_unboxed_int();
    }
}
*/

pub fn bounds_error_ints(v: &Value, idxs: &[usize]) -> Result<()> {
    let v = v.lock()?;
    let nidxs = idxs.len();
    let idxs = idxs.as_ptr() as *mut _;
    unsafe {
        jl_bounds_error_ints(v, idxs, nidxs);
    }
    Ok(())
}

pub fn eof_error() {
    unsafe {
        jl_eof_error();
    }
}

pub fn exception_occurred() -> Option<Value> {
    let raw = unsafe { jl_exception_occurred() };
    Value::new(raw).ok()
}

pub fn exception_clear() -> Option<Value> {
    let ret = exception_occurred();
    unsafe {
        jl_exception_clear();
    }
    ret
}
