
use std::fmt;

use smallvec::SmallVec;

use sys::*;
use value::{Value, JlValue};
use error::Result;
use sym::Symbol;
use datatype::Datatype;
use string::AsCString;

#[derive(Clone)]
pub enum Exception {
    Argument(Value),
    Bounds(Value),
    Composite(Value),
    Divide(Value),
    Domain(Value),
    EOF(Value),
    Error(Value),
    Inexact(Value),
    Init(Value),
    Interrupt(Value),
    InvalidState(Value),
    Key(Value),
    Load(Value),
    OutOfMemory(Value),
    ReadOnlyMemory(Value),
    Remote(Value),
    Method(Value),
    Overflow(Value),
    Parse(Value),
    System(Value),
    Type(Value),
    UndefRef(Value),
    UndefVar(Value),
    Unicode(Value),
    Unknown(Value),
}

impl Exception {
    pub fn occurred() -> Option<Exception> {
        let raw = unsafe { jl_exception_occurred() };
        Value::new(raw).and_then(Exception::with_value).ok()
    }
    
    pub fn clear() -> Option<Exception> {
        let ret = Exception::occurred();
        unsafe {
            jl_exception_clear();
        }
        ret
    }

    pub fn with_value(value: Value) -> Result<Exception> {
        let typename = value.typename()?;
        let ex = match typename.as_str() {
            "ArgumentError" => Exception::Argument(value),
            "BoundsError" => Exception::Bounds(value),
            "CompositeException" => Exception::Composite(value),
            "DivideError" => Exception::Divide(value),
            "DomainError" => Exception::Domain(value),
            "EOFError" => Exception::EOF(value),
            "ErrorException" => Exception::Error(value),
            "InexactError" => Exception::Inexact(value),
            "InitError" => Exception::Init(value),
            "InterruptException" => Exception::Interrupt(value),
            "InvalidStateException" => Exception::InvalidState(value),
            "KeyError" => Exception::Key(value),
            "LoadError" => Exception::Load(value),
            "OutOfMemoryError" => Exception::OutOfMemory(value),
            "ReadOnlyMemoryError" => Exception::ReadOnlyMemory(value),
            "RemoteException" => Exception::Remote(value),
            "MethodError" => Exception::Method(value),
            "OverflowError" => Exception::Overflow(value),
            "ParseError" => Exception::Parse(value),
            "SystemError" => Exception::System(value),
            "TypeError" => Exception::Type(value),
            "UndefRefError" => Exception::UndefRef(value),
            "UndefVarError" => Exception::UndefVar(value),
            "UnicodeError" => Exception::Unicode(value),
            _ => Exception::Unknown(value),
        };
        Ok(ex)
    }

    pub fn into_inner(self) -> Value {
        match self {
            Exception::Argument(value) => value,
            Exception::Bounds(value) => value,
            Exception::Composite(value) => value,
            Exception::Divide(value) => value,
            Exception::Domain(value) => value,
            Exception::EOF(value) => value,
            Exception::Error(value) => value,
            Exception::Inexact(value) => value,
            Exception::Init(value) => value,
            Exception::Interrupt(value) => value,
            Exception::InvalidState(value) => value,
            Exception::Key(value) => value,
            Exception::Load(value) => value,
            Exception::OutOfMemory(value) => value,
            Exception::ReadOnlyMemory(value) => value,
            Exception::Remote(value) => value,
            Exception::Method(value) => value,
            Exception::Overflow(value) => value,
            Exception::Parse(value) => value,
            Exception::System(value) => value,
            Exception::Type(value) => value,
            Exception::UndefRef(value) => value,
            Exception::UndefVar(value) => value,
            Exception::Unicode(value) => value,
            Exception::Unknown(value) => value,
        }
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
