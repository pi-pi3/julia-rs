
use std::convert::TryFrom;
use std::ffi::CStr;

use sys::*;
use error::{Result, Error};
use string::TryIntoString;
use api::{Function, Datatype, IntoSymbol};

pub trait JlValue<T>
where
    Self: Sized,
{
    unsafe fn new_unchecked(_inner: *mut T) -> Self;
    fn new(_inner: *mut T) -> Result<Self>;
    fn lock(&self) -> Result<*mut T>;
    fn into_inner(self) -> Result<*mut T>;

    fn typename(&self) -> Result<String> {
        let raw = self.lock()? as *mut jl_value_t;
        let t = unsafe { jl_typeof_str(raw) };
        jl_catch!();
        t.try_into_string()
    }

    fn datatype(&self) -> Result<Datatype> {
        let raw = self.lock()? as *mut jl_value_t;
        let dt = unsafe { jl_typeof(raw) };
        jl_catch!();
        Datatype::new(dt as *mut jl_datatype_t)
    }

    fn get<S: IntoSymbol>(&self, field: S) -> Result<Value> {
        let raw = self.lock()? as *mut jl_value_t;
        let field = field.into_symbol()?;
        let field = field.lock()?;
        let dt = self.datatype()?;
        let dt = dt.lock()?;
        let idx = unsafe { jl_field_index(dt, field, -1) };
        jl_catch!();

        if idx.is_negative() {
            return Err(Error::InvalidSymbol);
        }
        let idx = idx as usize;

        let value = unsafe { jl_get_nth_field(raw, idx) };
        jl_catch!();
        Value::new(value)
    }

    fn set<S: IntoSymbol>(&self, field: S, value: Value) -> Result<()> {
        let raw = self.lock()? as *mut jl_value_t;
        let field = field.into_symbol()?;
        let field = field.lock()?;
        let dt = self.datatype()?;
        let dt = dt.lock()?;
        let idx = unsafe { jl_field_index(dt, field, -1) };
        jl_catch!();

        if idx.is_negative() {
            return Err(Error::InvalidSymbol);
        }
        let idx = idx as usize;

        let value = value.lock()?;
        unsafe { jl_set_nth_field(raw, idx, value) };
        jl_catch!();
        Ok(())
    }

    fn from_value<U, A: JlValue<U>>(val: A) -> Result<Self> {
        let raw = val.into_inner()? as *mut T;
        Self::new(raw)
    }

    fn into_value<U, A: JlValue<U>>(self) -> Result<A> {
        let raw = self.into_inner()? as *mut U;
        A::new(raw)
    }
}

macro_rules! simple_jlvalue {
    ($name:ident, $type:ty) => {
        #[derive(Clone)]
        pub struct $name {
            _inner: ::std::rc::Rc<::std::sync::Mutex<::std::ptr::Unique<$type>>>,
        }

        impl $crate::api::JlValue<$type> for $name {
            unsafe fn new_unchecked(_inner: *mut $type) -> $name {
                $name {
                    _inner: ::std::rc::Rc::new(
                                ::std::sync::Mutex::new(
                                    ::std::ptr::Unique::new_unchecked(_inner)
                                )
                            ),
                }
            }

            fn new(_inner: *mut $type) -> $crate::error::Result<$name> {
                if _inner.is_null() {
                    Err($crate::error::Error::NullPointer)
                } else {
                    unsafe {
                        Ok($name::new_unchecked(_inner))
                    }
                }
            }

            fn lock(&self) -> $crate::error::Result<*mut $type> {
                self._inner
                    .lock()
                    .map(|ptr| ptr.as_ptr())
                    .map_err(From::from)
            }

            fn into_inner(self) -> $crate::error::Result<*mut $type> {
                ::std::rc::Rc::try_unwrap(self._inner)?
                    .into_inner()
                    .map(::std::ptr::Unique::as_ptr)
                    .map_err(From::from)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use $crate::api::JlValue;
                let typename = self.typename().map_err(|_| ::std::fmt::Error)?;
                write!(f, "{}({})", typename, self)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use ::std::convert::TryFrom;
                use $crate::api::JlValue;
                let jl_string = unsafe {
                    let name = ::std::ffi::CString::new("string")
                        .map_err(|_| ::std::fmt::Error)?;
                    let name = name.as_ptr();
                    $crate::sys::jl_get_function($crate::sys::jl_base_module, name)
                };
                jl_catch!(|ex -> ::std::fmt::Error| ::std::fmt::Error);
                let jl_string = $crate::api::Function::new(jl_string)
                    .map_err(|_| ::std::fmt::Error)?;

                let inner = self.lock()
                    .map_err(|_| ::std::fmt::Error)?;
                let value = $crate::api::Value::new(inner as *mut jl_value_t)
                    .map_err(|_| ::std::fmt::Error)?;

                let string = jl_string.call1(&value)
                    .map_err(|_| ::std::fmt::Error)?;
                let string = String::try_from(&string)
                    .map_err(|_| ::std::fmt::Error)?;

                write!(f, "{}", string)
            }
        }
    }
}

#[macro_export]
macro_rules! jlvalues {
    { $( pub struct $name:ident ($type:ty) );*; } => {
        $(
            simple_jlvalue!($name, $type);
        )*
    }
}

jlvalues! {
    pub struct Value(jl_value_t);
}

impl Value {
    pub fn nothing() -> Value {
        unsafe { Value::new_unchecked(jl_nothing) }
    }

    pub fn expand(&self) -> Result<Value> {
        let raw = self.lock()?;
        let raw = unsafe { jl_expand(raw) };;
        jl_catch!();
        Value::new(raw)
    }

    pub fn map<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(*mut jl_value_t) -> T,
    {
        self.lock().map(f)
    }

    pub fn map_or<T, F>(&self, f: F, optb: T) -> T
    where
        F: FnOnce(*mut jl_value_t) -> T,
    {
        self.lock().map(f).unwrap_or(optb)
    }

    pub fn map_or_else<T, F, O>(&self, f: F, op: O) -> T
    where
        F: FnOnce(*mut jl_value_t) -> T,
        O: FnOnce(Error) -> T,
    {
        self.lock().map(f).unwrap_or_else(op)
    }

    pub fn is_nothing(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_nothing(v) }, false)
    }
    pub fn is_tuple(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_tuple(v) }, false)
    }
    pub fn is_svec(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_svec(v) }, false)
    }
    pub fn is_simplevector(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_simplevector(v) }, false)
    }
    pub fn is_datatype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_datatype(v) }, false)
    }
    pub fn is_mutable(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_mutable(v) }, false)
    }
    pub fn is_mutable_datatype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_mutable_datatype(v) }, false)
    }
    pub fn is_immutable(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_immutable(v) }, false)
    }
    pub fn is_immutable_datatype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_immutable_datatype(v) }, false)
    }
    pub fn is_uniontype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uniontype(v) }, false)
    }
    pub fn is_typevar(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_typevar(v) }, false)
    }
    pub fn is_unionall(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_unionall(v) }, false)
    }
    pub fn is_typename(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_typename(v) }, false)
    }
    pub fn is_int8(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_int8(v) }, false)
    }
    pub fn is_int16(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_int16(v) }, false)
    }
    pub fn is_int32(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_int32(v) }, false)
    }
    pub fn is_int64(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_int64(v) }, false)
    }
    pub fn is_long(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_long(v) }, false)
    }
    pub fn is_uint8(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uint8(v) }, false)
    }
    pub fn is_uint16(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uint16(v) }, false)
    }
    pub fn is_uint32(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uint32(v) }, false)
    }
    pub fn is_uint64(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uint64(v) }, false)
    }
    pub fn is_ulong(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_ulong(v) }, false)
    }
    pub fn is_float16(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_float16(v) }, false)
    }
    pub fn is_float32(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_float32(v) }, false)
    }
    pub fn is_float64(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_float64(v) }, false)
    }
    pub fn is_bool(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_bool(v) }, false)
    }
    pub fn is_symbol(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_symbol(v) }, false)
    }
    pub fn is_ssavalue(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_ssavalue(v) }, false)
    }
    pub fn is_slot(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_slot(v) }, false)
    }
    pub fn is_expr(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_expr(v) }, false)
    }
    pub fn is_globalref(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_globalref(v) }, false)
    }
    pub fn is_labelnode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_labelnode(v) }, false)
    }
    pub fn is_gotonode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_gotonode(v) }, false)
    }
    pub fn is_quotenode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_quotenode(v) }, false)
    }
    pub fn is_newvarnode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_newvarnode(v) }, false)
    }
    pub fn is_linenode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_linenode(v) }, false)
    }
    pub fn is_method_instance(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_method_instance(v) }, false)
    }
    pub fn is_code_info(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_code_info(v) }, false)
    }
    pub fn is_method(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_method(v) }, false)
    }
    pub fn is_module(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_module(v) }, false)
    }
    pub fn is_mtable(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_mtable(v) }, false)
    }
    pub fn is_task(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_task(v) }, false)
    }
    pub fn is_string(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_string(v) }, false)
    }
    pub fn is_cpointer(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_cpointer(v) }, false)
    }
    pub fn is_pointer(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_pointer(v) }, false)
    }
    pub fn is_intrinsic(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_intrinsic(v) }, false)
    }
    // pub fn is_function(&self) -> bool { true

    pub fn is_kind(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_kind(v) }, false)
    }
    pub fn is_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_type(v) }, false)
    }
    pub fn is_primitivetype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_primitivetype(v) }, false)
    }
    pub fn is_structtype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_structtype(v) }, false)
    }
    pub fn isbits(&self) -> bool {
        self.map_or(|v| unsafe { jl_isbits(v) }, false)
    }
    pub fn is_abstracttype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_abstracttype(v) }, false)
    }
    pub fn is_array_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_array_type(v) }, false)
    }
    pub fn is_array(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_array(v) }, false)
    }
    pub fn is_cpointer_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_cpointer_type(v) }, false)
    }
    pub fn is_abstract_ref_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_abstract_ref_type(v) }, false)
    }
    pub fn is_tuple_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_tuple_type(v) }, false)
    }
    pub fn is_vecelement_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_vecelement_type(v) }, false)
    }
    pub fn is_type_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_type_type(v) }, false)
    }
    pub fn is_vararg_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_vararg_type(v) }, false)
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::nothing()
    }
}

macro_rules! box_simple {
    ($t1:ident) => {
        box_simple!($t1 => $t1, |val| { val } );
    };
    ($t1:ident => $t2:ident) => {
        box_simple!($t1 => $t2, |val| { val } );
    };
    ($t1:ty => $t2:ident) => {
        box_simple!($t1 => $t2, |val| { val } );
    };
    ($t1:ident, |$v:ident| $fn:expr) => {
        box_simple!($t1 => $t1, |$v| $fn);
    };
    ($t1:ident => $t2:ident, |$v:ident| $fn:expr) => {
        impl From<$t1> for Value {
            fn from($v: $t1) -> Value {
                unsafe { Value::new_unchecked(concat_idents!(jl_box_, $t2)($fn)) }
            }
        }
    };
    ($t1:ty => $t2:ident, |$v:ident| $fn:expr) => {
        impl From<$t1> for Value {
            fn from($v: $t1) -> Value {
                unsafe { Value::new_unchecked(concat_idents!(jl_box_, $t2)($fn)) }
            }
        }
    }
}

macro_rules! unbox_simple {
    ($t1:ty) => {
        unbox_simple!($t1 => $t1);
    };
    ($t1:ident => $t2:ty) => {
        unbox_simple!($t1 => $t2, |v| { v } );
    };
    ($t1:ident => $t2:ty, |$v:ident| $fn:expr) => {
        impl<'a> TryFrom<&'a Value> for $t2 {
            type Error = Error;
            fn try_from(val: &Value) -> Result<$t2> {
                let is_type = {
                    let inner = val.lock()?;
                    unsafe {
                        concat_idents!(jl_is_, $t1)(inner)
                    }
                };
                if is_type {
                    let ret = val.lock()
                        .map(|v| unsafe { concat_idents!(jl_unbox_, $t1)(v) })
                        .map_err(From::from);
                    jl_catch!();
                    match ret {
                        Ok($v) => Ok($fn),
                        Err(x) => Err(x),
                    }
                } else {
                    Err(Error::InvalidUnbox)
                }
            }
        }
    }
}

box_simple!(bool, |val| val as i8);
box_simple!(char, |val| val as u32);

box_simple!(i8 => int8);
box_simple!(i16 => int16);
box_simple!(i32 => int32);
box_simple!(i64 => int64);
box_simple!(isize => long);
box_simple!(u8 => uint8);
box_simple!(u16 => uint16);
box_simple!(u32 => uint32);
box_simple!(u64 => uint64);
box_simple!(usize => ulong);
box_simple!(f32 => float32);
box_simple!(f64 => float64);

unbox_simple!(bool => bool, |val| val != 0);
unbox_simple!(uint32 => char, |val| char::try_from(val)?);

unbox_simple!(int8 => i8);
unbox_simple!(int16 => i16);
unbox_simple!(int32 => i32);
unbox_simple!(int64 => i64);
unbox_simple!(long => isize);
unbox_simple!(uint8 => u8);
unbox_simple!(uint16 => u16);
unbox_simple!(uint32 => u32);
unbox_simple!(uint64 => u64);
unbox_simple!(ulong => usize);
unbox_simple!(float32 => f32);
unbox_simple!(float64 => f64);

impl<'a> TryFrom<&'a Value> for String {
    type Error = Error;
    fn try_from(val: &Value) -> Result<String> {
        if val.is_string() {
            let ret: *mut i8 = unsafe {
                let name = ::std::ffi::CString::new("pointer")?;
                let name = name.as_ptr();
                let jl_pointer = jl_get_function(jl_base_module, name);;
                jl_catch!();
                let jl_pointer = Function::new(jl_pointer)?;

                let cpointer = jl_pointer.call1(val)?;
                cpointer.lock()
                    .map(|v| unsafe { jl_unbox_voidpointer(v) as *mut i8 })?
            };
            jl_catch!();

            let cstr = unsafe {
                CStr::from_ptr(ret)
            };
            cstr.to_owned()
                .into_string()
                .map_err(From::from)
        } else {
            Err(Error::InvalidUnbox)
        }
    }
}
