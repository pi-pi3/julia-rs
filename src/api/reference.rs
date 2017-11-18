
use std::fmt;
use std::rc::Rc;
use std::sync::Mutex;
use std::ptr::Unique;
use std::convert::TryFrom;
use std::result::Result as StdResult;

use sys::*;
use error::{Result, Error};
use string::{IntoCString, TryIntoString};
use api::{Void, Pointer, Datatype, Function, IntoSymbol, Exception};

pub trait ToJulia {
    type Error;
    fn to_julia(self) -> StdResult<Ref, Self::Error>;
}

pub trait FromJulia: Sized {
    type Error;
    fn from_julia(jl_ref: &Ref) -> StdResult<Self, Self::Error>;
}

#[derive(Clone)]
pub struct Ref {
    pub(crate) inner: Rc<Mutex<Unique<Void>>>,
}

impl Ref {
    /// Construct a new Ref from a raw pointer obtained from Julia.
    unsafe fn new_unchecked(inner: Pointer) -> Ref {
        Ref { inner: Rc::new(Mutex::new(Unique::new_unchecked(inner))) }
    }

    /// Construct a new Ref from a raw pointer obtained from Julia while
    /// previously validating it.
    ///
    /// # Panics
    ///
    /// Panics if `inner` is a nul-pointer.
    pub fn new<T>(inner: *mut T) -> Self {
        if inner.is_null() {
            panic!("cannot use a nul-pointer")
        } else {
            unsafe { Ref::new_unchecked(inner as Pointer) }
        }
    }

    /// Nothing, Nil, Null, None.
    pub fn nothing() -> Ref {
        unsafe { Ref::new_unchecked(jl_nothing as Pointer) }
    }

    /// Safely borrow the unique pointer to a inner jl_value.
    ///
    /// # Errors
    ///
    /// Returns Error::PoisonError if the inner Mutex is poisoned.
    pub fn lock<T>(&self) -> Result<*mut T> {
        self.inner
            .lock()
            .map(|ptr| ptr.as_ptr() as *mut T)
            .map_err(From::from)
    }

    /// Take ownership of the inner jl_value.
    ///
    /// # Errors
    ///
    /// Returns Error::PoisonError if the inner Mutex is poisoned.
    /// Returns Error::ResourceInUse if this resource is borrowed somewhere
    /// else.
    pub fn into_inner<T>(self) -> Result<*mut T> {
        Rc::try_unwrap(self.inner)?
            .into_inner()
            .map(|ptr| ptr.as_ptr() as *mut T)
            .map_err(From::from)
    }

    /// Add a finalizer, a function that will be run when the object is
    /// collected.
    pub fn add_finalizer(&self, f: &Ref) -> Result<()> {
        let raw = self.lock()?;
        let f = f.lock()?;

        unsafe {
            jl_gc_add_finalizer(raw, f);
        }
        Ok(())
    }

    /// Consume and finalize self.
    pub fn finalize(self) -> Result<()> {
        let raw = self.into_inner()?;

        unsafe {
            jl_finalize(raw);
        }
        Ok(())
    }

    /// Returns the name of the type.
    pub fn typename(&self) -> Result<String> {
        let raw = self.lock()?;

        let t = unsafe { jl_typeof_str(raw) };
        Ok(t.try_into_string().unwrap())
    }

    /// Returns the type of the object as a Datatype.
    pub fn datatype(&self) -> Result<Datatype> {
        let raw = self.lock()?;

        let dt = unsafe { jl_typeof(raw) };
        Ok(Datatype(Ref::new(dt)))
    }

    /// Returns the value of a field if it exists.
    pub fn get<S: IntoSymbol>(&self, field: S) -> Result<Ref> {
        let field = field.into_symbol()?;
        let field = field.lock()?;
        let dt = self.datatype()?;
        let dt = dt.lock()?;
        let idx =
            except! {
            try {
                unsafe {
                    jl_field_index(dt, field, 1)
                }
            } catch Exception::Error(ex) => {
                rethrow!(Exception::Error(ex))
            }
        };

        let idx = idx as usize;
        let raw = self.lock()?;

        let value = unsafe { jl_get_nth_field(raw, idx) };
        Ok(Ref::new(value))
    }

    /// Sets the value of a field if it exists.
    pub fn set<S: IntoSymbol>(&self, field: S, value: &Ref) -> Result<()> {
        let field = field.into_symbol()?;
        let field = field.lock()?;
        let dt = self.datatype()?;
        let dt = dt.lock()?;
        let idx =
            except! {
            try {
                unsafe {
                    jl_field_index(dt, field, 1)
                }
            } catch Exception::Error(ex) => {
                rethrow!(Exception::Error(ex))
            }
        };

        let idx = idx as usize;
        let raw = self.lock()?;
        let value = value.lock()?;

        unsafe { jl_set_nth_field(raw, idx, value) };
        Ok(())
    }

    /// Applies function to the inner pointer.
    pub fn map<A, B, F>(&self, f: F) -> Result<B>
    where
        F: FnOnce(*mut A) -> B,
    {
        self.lock().map(f)
    }

    /// Applies function to the inner pointer and returns a default value if
    /// its poisoned.
    pub fn map_or<A, B, F>(&self, f: F, optb: B) -> B
    where
        F: FnOnce(*mut A) -> B,
    {
        self.lock().map(f).unwrap_or(optb)
    }

    /// Applies function to the inner pointer and executes a default function if
    /// its poisoned.
    pub fn map_or_else<A, B, F, O>(&self, f: F, op: O) -> B
    where
        F: FnOnce(*mut A) -> B,
        O: FnOnce(Error) -> B,
    {
        self.lock().map(f).unwrap_or_else(op)
    }

    /// Checks if the inner Mutex is poisoned.
    pub fn is_ok(&self) -> bool {
        !self.inner.is_poisoned()
    }

    /// Checks if the Ref is of a concrete Datatype.
    pub fn isa(&self, dt: &Datatype) -> Result<bool> {
        let raw = self.lock()?;
        let dt = dt.lock()?;

        let p = unsafe { jl_isa(raw, dt) != 0 };
        Ok(p)
    }

    /// Checks if the types of two Refs are equal.
    pub fn types_equal(&self, other: &Ref) -> Result<bool> {
        let raw = self.lock()?;
        let other = other.lock()?;

        let p = unsafe { jl_types_equal(raw, other) != 0 };
        Ok(p)
    }

    /// Checks if the value is a nothing.
    pub fn is_nothing(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_nothing(v) }, false)
    }
    /// Checks if the value is a tuple.
    pub fn is_tuple(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_tuple(v) }, false)
    }
    /// Checks if the value is a svec.
    pub fn is_svec(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_svec(v) }, false)
    }
    /// Checks if the value is a simplevector.
    pub fn is_simplevector(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_simplevector(v) }, false)
    }
    /// Checks if the value is a datatype.
    pub fn is_datatype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_datatype(v) }, false)
    }
    /// Checks if the value is a mutable.
    pub fn is_mutable(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_mutable(v) }, false)
    }
    /// Checks if the value is a mutable_datatype.
    pub fn is_mutable_datatype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_mutable_datatype(v) }, false)
    }
    /// Checks if the value is a immutable.
    pub fn is_immutable(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_immutable(v) }, false)
    }
    /// Checks if the value is a immutable_datatype.
    pub fn is_immutable_datatype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_immutable_datatype(v) }, false)
    }
    /// Checks if the value is a uniontype.
    pub fn is_uniontype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uniontype(v) }, false)
    }
    /// Checks if the value is a typevar.
    pub fn is_typevar(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_typevar(v) }, false)
    }
    /// Checks if the value is a unionall.
    pub fn is_unionall(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_unionall(v) }, false)
    }
    /// Checks if the value is a typename.
    pub fn is_typename(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_typename(v) }, false)
    }
    /// Checks if the value is a int8.
    pub fn is_int8(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_int8(v) }, false)
    }
    /// Checks if the value is a int16.
    pub fn is_int16(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_int16(v) }, false)
    }
    /// Checks if the value is a int32.
    pub fn is_int32(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_int32(v) }, false)
    }
    /// Checks if the value is a int64.
    pub fn is_int64(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_int64(v) }, false)
    }
    /// Checks if the value is a long.
    pub fn is_long(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_long(v) }, false)
    }
    /// Checks if the value is a uint8.
    pub fn is_uint8(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uint8(v) }, false)
    }
    /// Checks if the value is a uint16.
    pub fn is_uint16(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uint16(v) }, false)
    }
    /// Checks if the value is a uint32.
    pub fn is_uint32(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uint32(v) }, false)
    }
    /// Checks if the value is a uint64.
    pub fn is_uint64(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_uint64(v) }, false)
    }
    /// Checks if the value is a ulong.
    pub fn is_ulong(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_ulong(v) }, false)
    }
    /// Checks if the value is a float16.
    pub fn is_float16(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_float16(v) }, false)
    }
    /// Checks if the value is a float32.
    pub fn is_float32(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_float32(v) }, false)
    }
    /// Checks if the value is a float64.
    pub fn is_float64(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_float64(v) }, false)
    }
    /// Checks if the value is a bool.
    pub fn is_bool(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_bool(v) }, false)
    }
    /// Checks if the value is a symbol.
    pub fn is_symbol(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_symbol(v) }, false)
    }
    /// Checks if the value is a ssavalue.
    pub fn is_ssavalue(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_ssavalue(v) }, false)
    }
    /// Checks if the value is a slot.
    pub fn is_slot(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_slot(v) }, false)
    }
    /// Checks if the value is a expr.
    pub fn is_expr(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_expr(v) }, false)
    }
    /// Checks if the value is a globalref.
    pub fn is_globalref(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_globalref(v) }, false)
    }
    /// Checks if the value is a labelnode.
    pub fn is_labelnode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_labelnode(v) }, false)
    }
    /// Checks if the value is a gotonode.
    pub fn is_gotonode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_gotonode(v) }, false)
    }
    /// Checks if the value is a quotenode.
    pub fn is_quotenode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_quotenode(v) }, false)
    }
    /// Checks if the value is a newvarnode.
    pub fn is_newvarnode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_newvarnode(v) }, false)
    }
    /// Checks if the value is a linenode.
    pub fn is_linenode(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_linenode(v) }, false)
    }
    /// Checks if the value is a method_instance.
    pub fn is_method_instance(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_method_instance(v) }, false)
    }
    /// Checks if the value is a code_info.
    pub fn is_code_info(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_code_info(v) }, false)
    }
    /// Checks if the value is a method.
    pub fn is_method(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_method(v) }, false)
    }
    /// Checks if the value is a module.
    pub fn is_module(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_module(v) }, false)
    }
    /// Checks if the value is a mtable.
    pub fn is_mtable(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_mtable(v) }, false)
    }
    /// Checks if the value is a task.
    pub fn is_task(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_task(v) }, false)
    }
    /// Checks if the value is a string.
    pub fn is_string(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_string(v) }, false)
    }
    /// Checks if the value is a cpointer.
    pub fn is_cpointer(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_cpointer(v) }, false)
    }
    /// Checks if the value is a pointer.
    pub fn is_pointer(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_pointer(v) }, false)
    }
    /// Checks if the value is a intrinsic.
    pub fn is_intrinsic(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_intrinsic(v) }, false)
    }
    /// Checks if the value is a bits.
    pub fn is_bits(&self) -> bool {
        self.map_or(|v| unsafe { jl_isbits(v) }, false)
    }
    /// Checks if the value is a type.
    pub fn is_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_type(v) }, false)
    }
    /// Checks if the value is a kind.
    pub fn is_kind(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_kind(v) }, false)
    }
    /// Checks if the value is a primitivetype.
    pub fn is_primitivetype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_primitivetype(v) }, false)
    }
    /// Checks if the value is a structtype.
    pub fn is_structtype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_structtype(v) }, false)
    }
    /// Checks if the value is a array_type.
    pub fn is_array_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_array_type(v) }, false)
    }
    /// Checks if the value is a abstracttype.
    pub fn is_abstracttype(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_abstracttype(v) }, false)
    }
    /// Checks if the value is a array.
    pub fn is_array(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_array(v) }, false)
    }
    /// Checks if the value is a cpointer_type.
    pub fn is_cpointer_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_cpointer_type(v) }, false)
    }
    /// Checks if the value is a abstract_ref_type.
    pub fn is_abstract_ref_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_abstract_ref_type(v) }, false)
    }
    /// Checks if the value is a tuple_type.
    pub fn is_tuple_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_tuple_type(v) }, false)
    }
    /// Checks if the value is a vecelement_type.
    pub fn is_vecelement_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_vecelement_type(v) }, false)
    }
    /// Checks if the value is a type_type.
    pub fn is_type_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_type_type(v) }, false)
    }
    /// Checks if the value is a vararg_type.
    pub fn is_vararg_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_vararg_type(v) }, false)
    }
}

impl ToJulia for Ref {
    type Error = Error;
    fn to_julia(self) -> Result<Ref> {
        Ok(self)
    }
}

impl FromJulia for Ref {
    type Error = Error;
    fn from_julia(jl_ref: &Ref) -> Result<Ref> {
        Ok(jl_ref.clone())
    }
}

impl Default for Ref {
    fn default() -> Ref {
        Ref::nothing()
    }
}

impl fmt::Debug for Ref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typename = self.typename().map_err(|_| fmt::Error)?;
        write!(f, "{}({})", typename, self)
    }
}

impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let jl_string = unsafe {
            let name = b"string\0";
            let name = name.as_ptr();
            jl_get_function(jl_base_module, name as *mut _)
        };

        let jl_string = Function(Ref::new(jl_string));

        let string = jl_string.call1(self).map_err(|_| fmt::Error)?;
        let string = String::from_julia(&string).map_err(|_| fmt::Error)?;

        write!(f, "{}", string)
    }
}

// # Examples
// wrap_ref! { struct Wrapped; }
// wrap_ref! { struct Wrapped(Expr); }
// wrap_ref! { struct Wrapped(Expr, i64, f64); }
// wrap_ref! { struct Wrapped { i: i64, f: f64 } }
macro_rules! wrap_ref {
    { pub struct $name:ident; } => {
        wrap_ref! { pub struct $name(Ref, ); }
    };
    { pub struct $name:ident (Ref); } => {
        wrap_ref! { pub struct $name(Ref, ); }
    };
    { pub struct $name:ident(Ref, $( $field:ty ),*); } => {
        pub struct $name(pub $crate::api::Ref, $( $field ),*);

        impl ::std::ops::Deref for $name {
            type Target = $crate::api::Ref;
            fn deref(&self) -> &$crate::api::Ref {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $crate::api::Ref {
                &mut self.0
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use std::ops::Deref;
                write!(f, "{:?}", self.deref())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use std::ops::Deref;
                write!(f, "{}", self.deref())
            }
        }
    };
    {
        pub struct $name:ident {
            $( $field:ident : $type:ty ),*
        }
    } => {
        pub struct $name {
            pub inner: $crate::api::Ref,
            $( $field : $type ),*
        }

        impl ::std::ops::Deref for $name {
            type Target = $crate::api::Ref;
            fn deref(&self) -> &$crate::api::Ref {
                &self.inner
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $crate::api::Ref {
                &mut self.inner
            }
        }
    }
}

macro_rules! impl_to_julia {
    ($t1:ty => $t2:ident) => {
        impl_to_julia!($t1 => $t2, |val| { val } );
    };
    ($t1:ty => $t2:ident, |$v:ident| $fn:expr) => {
        impl ToJulia for $t1 {
            type Error = Error;
            fn to_julia(self) -> Result<Ref> {
                let $v = self;
                let jl_ref = unsafe { Ref::new(concat_idents!(jl_box_, $t2)($fn)) };
                Ok(jl_ref)
            }
        }
    }
}

macro_rules! impl_from_julia {
    ($t1:ident => $t2:ty) => {
        impl_from_julia!($t1 => $t2, |v| { v } );
    };
    ($t1:ident => $t2:ty, |$v:ident| $fn:expr) => {
        impl FromJulia for $t2 {
            type Error = Error;
            fn from_julia(jl_ref: &Ref) -> Result<$t2> {
                let raw = jl_ref.lock()?;

                let is_type = unsafe { concat_idents!(jl_is_, $t1)(raw) };

                if is_type {
                    let $v = unsafe { concat_idents!(jl_unbox_, $t1)(raw as *mut _) };
                    Ok($fn)
                } else {
                    Err(Error::InvalidUnbox)
                }
            }
        }
    }
}

impl_to_julia!(bool => bool, |val| val as i8);
impl_to_julia!(char => char, |val| val as u32);

impl_to_julia!(i8 => int8);
impl_to_julia!(i16 => int16);
impl_to_julia!(i32 => int32);
impl_to_julia!(i64 => int64);
impl_to_julia!(isize => long);
impl_to_julia!(u8 => uint8);
impl_to_julia!(u16 => uint16);
impl_to_julia!(u32 => uint32);
impl_to_julia!(u64 => uint64);
impl_to_julia!(usize => ulong);
impl_to_julia!(f32 => float32);
impl_to_julia!(f64 => float64);

impl<S: IntoCString> ToJulia for S {
    type Error = Error;
    fn to_julia(self) -> Result<Ref> {
        let cstr = self.into_cstring();
        let ptr = cstr.as_ptr();
        let raw = unsafe { jl_cstr_to_string(ptr) };
        let jl_ref = Ref::new(raw);
        Ok(jl_ref)
    }
}

impl_from_julia!(bool => bool, |val| val != 0);
impl_from_julia!(uint32 => char, |val| char::try_from(val)?);

impl_from_julia!(int8 => i8);
impl_from_julia!(int16 => i16);
impl_from_julia!(int32 => i32);
impl_from_julia!(int64 => i64);
impl_from_julia!(long => isize);
impl_from_julia!(uint8 => u8);
impl_from_julia!(uint16 => u16);
impl_from_julia!(uint32 => u32);
impl_from_julia!(uint64 => u64);
impl_from_julia!(ulong => usize);
impl_from_julia!(float32 => f32);
impl_from_julia!(float64 => f64);

impl FromJulia for String {
    type Error = Error;
    fn from_julia(jl_ref: &Ref) -> Result<String> {
        if jl_ref.is_string() {
            let raw = jl_ref.lock()?;
            let ptr = unsafe { jl_string_ptr(raw) };

            Ok(ptr.try_into_string().unwrap())
        } else {
            Err(Error::InvalidUnbox)
        }
    }
}
