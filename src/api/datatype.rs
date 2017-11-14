
//! Module providing wrappers for the native Julia type-types.

use std::ptr;

use sys::*;
use error::{Result, Error};
use api::{Value, JlValue, IntoSymbol, Array, Svec};

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

    pub fn any() -> Datatype { unsafe { Datatype::new_unchecked(jl_any_type) } }
    pub fn number() -> Datatype { unsafe { Datatype::new_unchecked(jl_number_type) } }
    pub fn signed() -> Datatype { unsafe { Datatype::new_unchecked(jl_signed_type) } }
    pub fn abstract_float() -> Datatype { unsafe { Datatype::new_unchecked(jl_floatingpoint_type) } }
    pub fn bool() -> Datatype { unsafe { Datatype::new_unchecked(jl_bool_type) } }
    pub fn char() -> Datatype { unsafe { Datatype::new_unchecked(jl_char_type) } }
    pub fn int8() -> Datatype { unsafe { Datatype::new_unchecked(jl_int8_type) } }
    pub fn uint8() -> Datatype { unsafe { Datatype::new_unchecked(jl_uint8_type) } }
    pub fn int16() -> Datatype { unsafe { Datatype::new_unchecked(jl_int16_type) } }
    pub fn uint16() -> Datatype { unsafe { Datatype::new_unchecked(jl_uint16_type) } }
    pub fn int32() -> Datatype { unsafe { Datatype::new_unchecked(jl_int32_type) } }
    pub fn uint32() -> Datatype { unsafe { Datatype::new_unchecked(jl_uint32_type) } }
    pub fn int64() -> Datatype { unsafe { Datatype::new_unchecked(jl_int64_type) } }
    pub fn uint64() -> Datatype { unsafe { Datatype::new_unchecked(jl_uint64_type) } }
    pub fn float16() -> Datatype { unsafe { Datatype::new_unchecked(jl_float16_type) } }
    pub fn float32() -> Datatype { unsafe { Datatype::new_unchecked(jl_float32_type) } }
    pub fn float64() -> Datatype { unsafe { Datatype::new_unchecked(jl_float64_type) } }
    pub fn void() -> Datatype { unsafe { Datatype::new_unchecked(jl_void_type) } }
    pub fn complex() -> Datatype { unsafe { Datatype::new_unchecked(jl_complex_type as *mut _) } }
    pub fn void_pointer() -> Datatype { unsafe { Datatype::new_unchecked(jl_voidpointer_type) } }
    pub fn pointer() -> Datatype { unsafe { Datatype::new_unchecked(jl_pointer_type as *mut _) } }
}

/// Type for constructing new primitive, abstract or compound types.
pub struct TypeBuilder {
    name: *mut jl_sym_t,
    supertype: *mut jl_datatype_t,
    params: *mut jl_svec_t,
    fnames: *mut jl_svec_t,
    ftypes: *mut jl_svec_t,
    nbits: usize,
    abstrac: bool,
    mutable: bool,
    ninitialized: bool,
    primitive: bool,
    err: Option<Error>,
}

impl TypeBuilder {
    /// Construct a new default TypeBuilder;
    pub fn new() -> TypeBuilder {
        TypeBuilder {
            name: ptr::null_mut(),
            supertype: unsafe { jl_any_type },
            params: unsafe { jl_emptysvec },
            fnames: unsafe { jl_emptysvec },
            ftypes: unsafe { jl_emptysvec },
            nbits: 0,
            abstrac: false,
            mutable: false,
            ninitialized: false,
            primitive: false,
            err: None,
        }
    }

    /// Get the error if it occurred.
    pub fn err(&self) -> Option<&Error> {
        self.err.as_ref()
    }

    /// Check if any error occurred.
    pub fn is_err(&self) -> bool {
        self.err.is_some()
    }

    /// Builds the Type. If any errors occurred previously, they will be returned here.
    pub fn build(self) -> Result<Datatype> {
        if let Some(err) = self.err {
            return Err(err);
        }

        if self.primitive {
            let raw = unsafe {
                jl_new_primitivetype(self.name as *mut _, self.supertype, self.params, self.nbits)
            };
            jl_catch!();
            Datatype::new(raw)
        } else {
            let raw = unsafe {
                jl_new_datatype(self.name, self.supertype,
                                self.params, self.fnames,
                                self.ftypes, self.abstrac as i32,
                                self.mutable as i32, self.ninitialized as i32)
            };
            jl_catch!();
            Datatype::new(raw)
        }
    }

    /// Sets the name.
    pub fn name<S: IntoSymbol>(mut self, name: S) -> TypeBuilder {
        let name = name
            .into_symbol();

        if let Err(err) = name {
            self.err = Some(err);
            return self;
        }

        let name = name
            .unwrap()
            .into_inner();

        self.name = match name {
            Ok(name) => name,
            Err(err) => {
                self.err = Some(err);
                return self;
            }
        };
        self
    }

    /// Sets the supertype. Must be an abstract.
    pub fn supertype(mut self, supertype: &Datatype) -> TypeBuilder {
        self.supertype = match supertype.lock() {
            Ok(supertype) => supertype,
            Err(err) => {
                self.err = Some(err);
                return self;
            }
        };
        self
    }

    pub fn params(mut self, params: &Svec) -> TypeBuilder {
        self.params = match params.lock() {
            Ok(params) => params,
            Err(err) => {
                self.err = Some(err);
                return self;
            }
        };
        self
    }

    /// Sets the names of the fields.
    pub fn fnames(mut self, fnames: &Svec) -> TypeBuilder {
        self.fnames = match fnames.lock() {
            Ok(fnames) => fnames,
            Err(err) => {
                self.err = Some(err);
                return self;
            }
        };
        self
    }

    /// Sets the types of the fields.
    pub fn ftypes(mut self, ftypes: &Svec) -> TypeBuilder {
        self.ftypes = match ftypes.lock() {
            Ok(ftypes) => ftypes,
            Err(err) => {
                self.err = Some(err);
                return self;
            }
        };
        self
    }

    /// Sets the number of bits in a primitive. Must be a multiple of 8.
    pub fn nbits(mut self, nbits: usize) -> TypeBuilder {
        self.nbits = nbits;
        self
    }

    /// Sets whether the type is abstract.
    pub fn abstrac(mut self, abstrac: bool) -> TypeBuilder {
        self.abstrac = abstrac;
        self
    }

    /// Sets whether the struct is mutable.
    pub fn mutable(mut self, mutable: bool) -> TypeBuilder {
        self.mutable = mutable;
        self
    }

    pub fn ninitialized(mut self, ninitialized: bool) -> TypeBuilder {
        self.ninitialized = ninitialized;
        self
    }

    /// Sets whether the type is a primitive.
    pub fn primitive(mut self, primitive: bool) -> TypeBuilder {
        self.primitive = primitive;
        self
    }
}

impl Default for TypeBuilder {
    fn default() -> TypeBuilder {
        TypeBuilder::new()
    }
}

/// Create a new Julia type using a Rust-like syntax.
///
/// # Syntax
///
/// ## Primitive type
/// ```
/// type <name> = Bits<N> where N: <bits> [ , Self: <supertype> ];
/// ```
///
/// ## Abstract type
/// ```
/// trait <name> [ : <supertype> ];
/// ```
///
/// ## Struct
/// ```
/// [mut] struct <name> [ : <supertype> ];
/// ```
/// **or**
/// ```
/// [mut] struct <name> {
///     (
///         <fname>: <ftype>,
///     )*
/// } [ : <supertype> ]
/// ```
#[macro_export]
macro_rules! jl_type {
    { type $name:ident = Bits<N> where N: $nbits:expr; } => {
        jl_type! { type $name = Bits<N> where N: $nbits, Self : Datatype::any(); }
    };
    { type $name:ident = Bits<N> where N: $nbits:expr, Self : $supertype:expr; } => {
        TypeBuilder::new()
            .primitive(true)
            .name(stringify!($name))
            .supertype(&$supertype)
            .nbits($nbits)
            .build()
    };
    { trait $name:ident; } => {
        jl_type! { type $name: Datatype::any(); }
    };
    { trait $name:ident : $supertype:expr; } => {
        TypeBuilder::new()
            .abstrac(true)
            .name(stringify!($name))
            .supertype(&$supertype)
            .build()
    };
    { struct $name:ident; } => {
        jl_type! { struct $name: Datatype::any(); }
    };
    { struct $name:ident : $supertype:expr; } => {
        TypeBuilder::new()
            .name(stringify!($name))
            .supertype(&$supertype)
            .build()
    };
    {
        struct $name:ident {
            $(
                $fname:ident : $ftype:expr
            ),*
        }
    } => {
        jl_type! {
            struct $name {
                $(
                    $fname : $ftype,
                )*
            } : Datatype::any()
        }
    };
    {
        struct $name:ident {
            $(
                $fname:ident : $ftype:expr,
            )*
        } : $supertype:expr
    } => {
        {
            use $crate::error::Result;
            use $crate::api::{IntoSymbol, Datatype};

            fn build() -> Result<Datatype> {
                TypeBuilder::new()
                    .name(stringify!($name))
                    .supertype(&$supertype)
                    .fnames(&jlvec![
                            $(
                                Value::from_value(
                                    stringify!($fname).into_symbol()?
                                )?
                            ),*
                        ]?)
                    .ftypes(&jlvec![
                            $( Value::from_value($ftype)? ),*
                        ]?)
                    .build()
            }

            build()
        }
    };
    { mut struct $name:ident; } => {
        jl_type! { mut struct $name: Datatype::any(); }
    };
    { mut struct $name:ident : $supertype:expr; } => {
        TypeBuilder::new()
            .mutable(true)
            .name(stringify!($name))
            .supertype(&$supertype)
            .build()
    };
    {
        mut struct $name:ident {
            $(
                $fname:ident : $ftype:expr,
            )*
        }
    } => {
        jl_type! {
            mut struct $name {
            $(
                $fname : $ftype,
            )*
            } : Datatype::any()
        }
    };
    {
        mut struct $name:ident {
            $(
                $fname:ident : $ftype:expr,
            )*
        } : $supertype:expr
    } => {
        {
            use $crate::error::Result;
            use $crate::api::{IntoSymbol, Datatype};

            fn build() -> Result<Datatype> {
                TypeBuilder::new()
                    .mutable(true)
                    .name(stringify!($name))
                    .supertype(&$supertype)
                    .fnames(&jlvec![
                            $(
                                Value::from_value(
                                    stringify!($fname).into_symbol()?
                                )?
                            ),*
                        ]?)
                    .ftypes(&jlvec![
                            $( Value::from_value($ftype)? ),*
                        ]?)
                    .build()
            }

            build()
        }
    };
}
