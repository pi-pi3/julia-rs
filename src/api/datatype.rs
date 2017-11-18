
//! Module providing wrappers for the native Julia type-types.

use std::ptr;
use std::result;
use std::convert::TryFrom;

use sys::*;
use error::{Result, Error};
use api::{Ref, IntoSymbol, Array, Svec, Exception};

#[derive(Clone, Copy, Hash, PartialEq, Debug)]
pub enum VarargKind {
    None,
    Int,
    Bound,
    Unbound,
}

impl TryFrom<u32> for VarargKind {
    type Error = ();
    fn try_from(kind: u32) -> result::Result<VarargKind, ()> {
        match kind {
            0 => Ok(VarargKind::None),
            1 => Ok(VarargKind::Int),
            2 => Ok(VarargKind::Bound),
            3 => Ok(VarargKind::Unbound),
            _ => Err(()),
        }
    }
}

wrap_ref! { pub struct Type(Ref); }
wrap_ref! { pub struct Datatype(Ref); }
wrap_ref! { pub struct Union(Ref); }
wrap_ref! { pub struct UnionAll(Ref); }
wrap_ref! { pub struct Tuple(Ref); }

impl Type {
    /// Creates a new Julia array of this type.
    pub fn new_array<I>(&self, params: I) -> Result<Array>
    where
        I: IntoIterator<Item = Ref>,
    {
        let mut paramv = vec![];
        for p in params {
            paramv.push(p.lock()?);
        }

        let dt = self.lock()?;
        let array =
            except! {
            try {
                unsafe { jl_alloc_array_1d(dt, paramv.len()) }
            } catch Exception::Error(ex) => {
                rethrow!(Exception::Error(ex))
            }
        };

        except! {
            try {
                for (i, p) in paramv.into_iter().enumerate() {
                    unsafe {
                        jl_arrayset(array, p, i);
                    }
                }
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        }

        Ok(Array(Ref::new(array)))
    }

    pub fn apply_type<'a, I>(&self, params: I) -> Result<Type>
    where
        I: IntoIterator<Item = &'a Ref>,
    {
        let mut paramv = vec![];
        for p in params {
            paramv.push(p.lock()?);
        }
        let nparam = paramv.len();
        let paramv = paramv.as_mut_ptr();

        let tc = self.lock()?;
        let raw =
            except! {
            try {
                unsafe {
                    jl_apply_type(tc, paramv, nparam)
                }
            } catch Exception::Error(ex) => {
                rethrow!(Exception::Error(ex))
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        };
        Ok(Type(Ref::new(raw)))
    }

    pub fn apply_type1(&self, p1: &Ref) -> Result<Type> {
        let tc = self.lock()?;
        let p1 = p1.lock()?;

        let raw =
            except! {
            try {
                unsafe {
                    jl_apply_type1(tc, p1)
                }
            } catch Exception::Error(ex) => {
                rethrow!(Exception::Error(ex))
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        };
        Ok(Type(Ref::new(raw)))
    }

    pub fn apply_type2(&self, p1: &Ref, p2: &Ref) -> Result<Type> {
        let tc = self.lock()?;
        let p1 = p1.lock()?;
        let p2 = p2.lock()?;

        let raw =
            except! {
            try {
                unsafe {
                    jl_apply_type2(tc, p1, p2)
                }
            } catch Exception::Error(ex) => {
                rethrow!(Exception::Error(ex))
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        };
        Ok(Type(Ref::new(raw)))
    }

    pub fn unwrap_vararg(&self) -> Result<Type> {
        let inner = self.lock()?;

        let raw = unsafe { jl_unwrap_vararg(inner) };
        Ok(Type(Ref::new(raw)))
    }

    pub fn vararg_kind(&self) -> Result<VarargKind> {
        let inner = self.lock()?;

        let kind = unsafe { jl_vararg_kind(inner) };
        Ok(VarargKind::try_from(kind).unwrap())
    }

    /// Checks if the value is a leaf type, i.e. not abstract and concrete.
    pub fn is_leaf_type(&self) -> bool {
        self.map_or(|v| unsafe { jl_is_leaf_type(v) != 0 }, false)
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

impl Datatype {
    /// Creates a new Julia struct of this type.
    pub fn new_struct<'a, I>(&self, params: I) -> Result<Ref>
    where
        I: IntoIterator<Item = &'a Ref>,
    {
        let mut paramv = vec![];
        for p in params {
            paramv.push(p.lock()?);
        }
        let nparam = paramv.len();
        let paramv = paramv.as_mut_ptr();

        let dt = self.lock()?;
        let value =
            except! {
            try {
                unsafe {
                    jl_new_structv(dt, paramv, nparam as u32)
                }
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        };
        Ok(Ref::new(value))
    }

    /// Creates a new Julia primitive of this type.
    pub fn new_bits<T: Into<Vec<u8>>>(&self, data: T) -> Result<Ref> {
        let data = data.into();
        let bits = data.as_ptr();

        let dt = self.lock()?;
        let value = unsafe { jl_new_bits(dt, bits as *mut _) };
        Ok(Ref::new(value))
    }

    pub fn any() -> Datatype {
        Datatype(Ref::new(unsafe { jl_any_type }))
    }
    pub fn number() -> Datatype {
        Datatype(Ref::new(unsafe { jl_number_type }))
    }
    pub fn signed() -> Datatype {
        Datatype(Ref::new(unsafe { jl_signed_type }))
    }
    pub fn abstract_float() -> Datatype {
        Datatype(Ref::new(unsafe { jl_floatingpoint_type }))
    }
    pub fn bool() -> Datatype {
        Datatype(Ref::new(unsafe { jl_bool_type }))
    }
    pub fn char() -> Datatype {
        Datatype(Ref::new(unsafe { jl_char_type }))
    }
    pub fn int8() -> Datatype {
        Datatype(Ref::new(unsafe { jl_int8_type }))
    }
    pub fn uint8() -> Datatype {
        Datatype(Ref::new(unsafe { jl_uint8_type }))
    }
    pub fn int16() -> Datatype {
        Datatype(Ref::new(unsafe { jl_int16_type }))
    }
    pub fn uint16() -> Datatype {
        Datatype(Ref::new(unsafe { jl_uint16_type }))
    }
    pub fn int32() -> Datatype {
        Datatype(Ref::new(unsafe { jl_int32_type }))
    }
    pub fn uint32() -> Datatype {
        Datatype(Ref::new(unsafe { jl_uint32_type }))
    }
    pub fn int64() -> Datatype {
        Datatype(Ref::new(unsafe { jl_int64_type }))
    }
    pub fn uint64() -> Datatype {
        Datatype(Ref::new(unsafe { jl_uint64_type }))
    }
    pub fn float16() -> Datatype {
        Datatype(Ref::new(unsafe { jl_float16_type }))
    }
    pub fn float32() -> Datatype {
        Datatype(Ref::new(unsafe { jl_float32_type }))
    }
    pub fn float64() -> Datatype {
        Datatype(Ref::new(unsafe { jl_float64_type }))
    }
    pub fn void() -> Datatype {
        Datatype(Ref::new(unsafe { jl_void_type }))
    }
    pub fn complex() -> Datatype {
        Datatype(Ref::new(unsafe { jl_complex_type }))
    }
    pub fn void_pointer() -> Datatype {
        Datatype(Ref::new(unsafe { jl_voidpointer_type }))
    }
    pub fn pointer() -> Datatype {
        Datatype(Ref::new(unsafe { jl_pointer_type }))
    }
}

impl Default for Datatype {
    fn default() -> Datatype {
        Datatype::any()
    }
}

impl Union {
    /// Create a union of types.
    pub fn union<'a, I>(ts: I) -> Result<Union>
    where
        I: IntoIterator<Item = &'a Datatype>,
    {
        let mut vec = vec![];
        for t in ts {
            vec.push(t.lock()?);
        }
        let n = vec.len();
        let ts_ptr = vec.as_mut_ptr();

        let raw =
            except! {
            try {
                unsafe { jl_type_union(ts_ptr, n) }
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        };
        Ok(Union(Ref::new(raw)))
    }

    /// Get the union that is an intersection of two types.
    pub fn intersection(a: &Union, b: &Union) -> Result<Union> {
        let a = a.lock()?;
        let b = b.lock()?;

        let raw =
            except! {
            try {
                unsafe { jl_type_intersection(a, b) }
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        };
        Ok(Union(Ref::new(raw)))
    }

    /// Check if the intersection of two unions is empty.
    pub fn has_empty_intersection(a: &Union, b: &Union) -> Result<bool> {
        let a = a.lock()?;
        let b = b.lock()?;

        let p = unsafe { jl_has_empty_intersection(a, b) };
        Ok(p != 0)
    }
}

impl UnionAll {
    /// Instantiate a UnionAll into a more concrete type.
    /// Not guaranteed to be a concrete datatype.
    pub fn instantiate(&self, p: &Ref) -> Result<Type> {
        let inner = self.lock()?;
        let p = p.lock()?;

        let raw =
            except! {
            try {
                unsafe { jl_instantiate_unionall(inner, p) }
            } catch Exception::Error(ex) => {
                rethrow!(Exception::Error(ex))
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        };
        Ok(Type(Ref::new(raw)))
    }
}

impl Tuple {
    pub fn apply(params: &Svec) -> Result<Tuple> {
        let params = params.lock()?;

        let raw =
            except! {
            try {
                unsafe { jl_apply_tuple_type(params) }
            } catch Exception::Error(ex) => {
                rethrow!(Exception::Error(ex))
            } catch Exception::Type(ex) => {
                rethrow!(Exception::Type(ex))
            }
        };
        Ok(Tuple(Ref::new(raw)))
    }
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
            Ok(Datatype(Ref::new(raw)))
        } else {
            let raw =
                except! {
                try {
                    unsafe {
                        jl_new_datatype(
                            self.name,
                            self.supertype,
                            self.params,
                            self.fnames,
                            self.ftypes,
                            self.abstrac as i32,
                            self.mutable as i32,
                            self.ninitialized as i32,
                        )
                    }
                } catch Exception::Error(ex) => {
                    rethrow!(Exception::Error(ex))
                }
            };
            Ok(Datatype(Ref::new(raw)))
        }
    }

    /// Sets the name.
    pub fn name<S: IntoSymbol>(mut self, name: S) -> TypeBuilder {
        let name = name.into_symbol();

        if let Err(err) = name {
            self.err = Some(err);
            return self;
        }

        let name = name.unwrap().lock();

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
                                Ref::from_value(
                                    stringify!($fname).into_symbol()?
                                )?
                            ),*
                        ]?)
                    .ftypes(&jlvec![
                            $( Ref::from_value($ftype)? ),*
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
                                Ref::from_value(
                                    stringify!($fname).into_symbol()?
                                )?
                            ),*
                        ]?)
                    .ftypes(&jlvec![
                            $( Ref::from_value($ftype)? ),*
                        ]?)
                    .build()
            }

            build()
        }
    };
}
