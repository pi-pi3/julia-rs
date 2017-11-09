
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

use smallvec::SmallVec;

use sys::*;
use error::Result;
use string::IntoCString;
use super::{Value, JlValue, Symbol, Datatype};

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
    pub fn occurred() -> bool {
        unsafe { !jl_exception_occurred().is_null() }
    }

    pub fn catch() -> Option<Exception> {
        let raw = unsafe { jl_exception_occurred() };
        unsafe {
            jl_exception_clear();
        }
        Value::new(raw).and_then(Exception::with_value).ok()
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

    pub fn inner_ref(&self) -> &Value {
        match *self {
            Exception::Argument(ref value) => value,
            Exception::Bounds(ref value) => value,
            Exception::Composite(ref value) => value,
            Exception::Divide(ref value) => value,
            Exception::Domain(ref value) => value,
            Exception::EOF(ref value) => value,
            Exception::Error(ref value) => value,
            Exception::Inexact(ref value) => value,
            Exception::Init(ref value) => value,
            Exception::Interrupt(ref value) => value,
            Exception::InvalidState(ref value) => value,
            Exception::Key(ref value) => value,
            Exception::Load(ref value) => value,
            Exception::OutOfMemory(ref value) => value,
            Exception::ReadOnlyMemory(ref value) => value,
            Exception::Remote(ref value) => value,
            Exception::Method(ref value) => value,
            Exception::Overflow(ref value) => value,
            Exception::Parse(ref value) => value,
            Exception::System(ref value) => value,
            Exception::Type(ref value) => value,
            Exception::UndefRef(ref value) => value,
            Exception::UndefVar(ref value) => value,
            Exception::Unicode(ref value) => value,
            Exception::Unknown(ref value) => value,
        }
    }

    pub fn inner_mut(&mut self) -> &mut Value {
        match *self {
            Exception::Argument(ref mut value) => value,
            Exception::Bounds(ref mut value) => value,
            Exception::Composite(ref mut value) => value,
            Exception::Divide(ref mut value) => value,
            Exception::Domain(ref mut value) => value,
            Exception::EOF(ref mut value) => value,
            Exception::Error(ref mut value) => value,
            Exception::Inexact(ref mut value) => value,
            Exception::Init(ref mut value) => value,
            Exception::Interrupt(ref mut value) => value,
            Exception::InvalidState(ref mut value) => value,
            Exception::Key(ref mut value) => value,
            Exception::Load(ref mut value) => value,
            Exception::OutOfMemory(ref mut value) => value,
            Exception::ReadOnlyMemory(ref mut value) => value,
            Exception::Remote(ref mut value) => value,
            Exception::Method(ref mut value) => value,
            Exception::Overflow(ref mut value) => value,
            Exception::Parse(ref mut value) => value,
            Exception::System(ref mut value) => value,
            Exception::Type(ref mut value) => value,
            Exception::UndefRef(ref mut value) => value,
            Exception::UndefVar(ref mut value) => value,
            Exception::Unicode(ref mut value) => value,
            Exception::Unknown(ref mut value) => value,
        }
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

impl Deref for Exception {
    type Target = Value;
    fn deref(&self) -> &Value {
        self.inner_ref()
    }
}

impl DerefMut for Exception {
    fn deref_mut(&mut self) -> &mut Value {
        self.inner_mut()
    }
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typename = self.typename().map_err(|_| fmt::Error)?;
        write!(f, "{}", typename)
    }
}

pub fn error<S: IntoCString>(string: S) {
    let string = string.into_cstring();
    let string = string.as_ptr();
    unsafe {
        jl_error(string);
    }
}

pub fn error_format(args: fmt::Arguments) {
    error(fmt::format(args).into_cstring());
}

pub fn exception<S: IntoCString>(ty: &Datatype, string: S) -> Result<()> {
    let ty = ty.lock()?;
    let string = string.into_cstring();
    let string = string.as_ptr();
    unsafe {
        jl_exceptionf(ty, string);
    }
    Ok(())
}

pub fn exception_format(ty: &Datatype, args: fmt::Arguments) -> Result<()> {
    exception(ty, fmt::format(args).into_cstring())
}

pub fn too_few_args<S: IntoCString>(fname: S, min: usize) {
    let fname = fname.into_cstring();
    let fname = fname.as_ptr();
    unsafe {
        jl_too_few_args(fname, min as i32);
    }
}

pub fn too_many_args<S: IntoCString>(fname: S, max: usize) {
    let fname = fname.into_cstring();
    let fname = fname.as_ptr();
    unsafe {
        jl_too_many_args(fname, max as i32);
    }
}

pub fn type_error<S: IntoCString>(fname: S, expected: &Value, got: &Value) -> Result<()> {
    let fname = fname.into_cstring();
    let fname = fname.as_ptr();
    let expected = expected.lock()?;
    let got = got.lock()?;
    unsafe {
        jl_type_error(fname, expected, got);
    }
    Ok(())
}

pub fn type_error_rt<S: IntoCString>(fname: S, context: S, ty: &Value, got: &Value) -> Result<()> {
    let fname = fname.into_cstring();
    let fname = fname.as_ptr();
    let context = context.into_cstring();
    let context = context.as_ptr();
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
