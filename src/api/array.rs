
//! Module providing wrappers for iteratable sequences.

use std::slice;

use sys::*;
use error::Result;
use api::{Pointer, Ref};

wrap_ref! { pub struct Array(Ref); }
wrap_ref! { pub struct ByteArray(Ref); }
wrap_ref! { pub struct Svec(Ref); }

impl Array {
    /// Returns the length of the Array.
    pub fn len(&self) -> Result<usize> {
        let len = unsafe { jl_array_len(self.lock()?) };
        Ok(len)
    }

    pub fn dim(&self, i: usize) -> Result<usize> {
        let dim = unsafe { jl_array_dim(self.lock()?, i) };
        Ok(dim)
    }

    pub fn dim0(&self) -> Result<usize> {
        self.dim(0)
    }

    pub fn nrows(&self) -> Result<usize> {
        let nrows = unsafe { jl_array_nrows(self.lock()?) };
        Ok(nrows)
    }

    pub fn ndims(&self) -> Result<usize> {
        let ndims = unsafe { jl_array_ndims(self.lock()?) };
        Ok(ndims)
    }

    /// Checks if the Array is empty.
    pub fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) == 0
    }

    /// Constructs a Vec of Refs from the Array.
    pub fn as_vec(&self) -> Result<Vec<Ref>> {
        let len = self.len()?;
        let ptr = unsafe { jl_array_data(self.lock()?) };
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        let vec = slice
            .iter()
            .map(|raw| Ref::new(raw as *const _ as Pointer))
            .collect();
        Ok(vec)
    }

    /// Returns the value at a specified index.
    pub fn index(&self, idx: usize) -> Result<Ref> {
        let raw = unsafe { jl_array_ptr_ref(self.lock()?, idx) };
        Ok(Ref::new(raw))
    }

    /// Sets the value at a specified index.
    pub fn index_set(&self, idx: usize, x: &Ref) -> Result<()> {
        unsafe {
            jl_array_ptr_set(self.lock()?, idx, x.lock()?);
        }
        Ok(())
    }
}

impl ByteArray {
    /// Returns the length of the ByteArray.
    pub fn len(&self) -> Result<usize> {
        let len = unsafe { jl_array_len(self.lock()?) };
        Ok(len)
    }

    /// Checks if the ByteArray is empty.
    pub fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) == 0
    }

    /// Constructs a slice of bytes without allocating new space.
    pub fn as_slice(&self) -> Result<&[u8]> {
        let len = self.len()?;
        let ptr = unsafe { jl_array_data(self.lock()?) as *mut u8 };
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        Ok(slice)
    }

    /// Constructs a Vec of Refs from the ByteArray.
    pub fn as_vec(&self) -> Result<Vec<u8>> {
        self.as_slice().map(|s| s.to_vec())
    }

    /// Returns the value at a specified index.
    pub fn index(&self, idx: usize) -> Result<u8> {
        let byte = unsafe { jl_array_uint8_ref(self.lock()?, idx) };
        Ok(byte)
    }

    /// Sets the value at a specified index.
    pub fn index_set(&self, idx: usize, x: u8) -> Result<()> {
        unsafe {
            jl_array_uint8_set(self.lock()?, idx, x);
        }
        Ok(())
    }
}

impl Svec {
    /// Returns the length of the Svec.
    pub fn len(&self) -> Result<usize> {
        let len = unsafe { jl_svec_len(self.lock()?) };
        Ok(len)
    }

    /// Checks if the Svec is empty.
    pub fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) == 0
    }

    /// Constructs a Vec of Refs from the Svec.
    pub fn as_vec(&self) -> Result<Vec<Ref>> {
        let len = self.len()?;
        let ptr = unsafe { jl_svec_data(self.lock()?) };
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        let vec = slice
            .iter()
            .map(|raw| Ref::new(*raw))
            .collect();
        Ok(vec)
    }

    /// Returns the value at a specified index.
    pub fn index(&self, idx: usize) -> Result<Ref> {
        let raw = unsafe { jl_svecref(self.lock()?, idx) };
        Ok(Ref::new(raw))
    }

    /// Sets the value at a specified index.
    pub fn index_set(&self, idx: usize, x: &Ref) -> Result<()> {
        unsafe {
            jl_svecset(self.lock()?, idx, x.lock()?);
        }
        Ok(())
    }
}

/// Creates a new Svec.
#[macro_export]
macro_rules! jlvec {
    [] => {
        {
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let raw = unsafe { $crate::sys::jl_svec(0) };
                Ok($crate::api::Svec($crate::api::Ref::new(raw)))
            }

            svec()
        }
    };
    [$elem:expr] => {
        {
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                use $crate::api::ToJulia;
                let elem = $elem.to_julia()?;
                let raw = unsafe {
                    $crate::sys::jl_svec1(elem.lock()?)
                };
                Ok($crate::api::Svec($crate::api::Ref::new(raw)))
            }

            svec()
        }
    };
    [$elem1:expr, $elem2:expr] => {
        {
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                use $crate::api::ToJulia;
                let elem1 = $elem1.to_julia()?;
                let elem2 = $elem2.to_julia()?;
                let raw = unsafe {
                    $crate::sys::jl_svec2(elem1.lock()?, elem2.lock()?)
                };
                Ok($crate::api::Svec($crate::api::Ref::new(raw)))
            }

            svec()
        }
    };
    [$( $elem:expr ),+] => {
        {
            let mut count = 0;
            #[allow(unknown_lints)]
            #[allow(no_effect)]
            {
                $(
                    || $elem;
                    count += 1;
                )+
            }

            fn svec(count: usize) -> $crate::error::Result<$crate::api::Svec> {
                use $crate::api::ToJulia;
                let raw = unsafe {
                    $crate::sys::jl_svec(count,
                                         $(
                                             $elem.to_julia()?.lock()? as *mut $crate::sys::jl_value_t
                                         ),+
                                         )
                };
                Ok($crate::api::Svec($crate::api::Ref::new(raw)))
            }

            svec(count)
        }
    };
    [$elem:expr; $n:expr] => {
        {
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                use $crate::api::ToJulia;
                let elem = $elem.to_julia()?;
                let raw = unsafe {
                    $crate::sys::jl_svec_fill($n, elem.lock()?)
                };
                Ok($crate::api::Svec($crate::api::Ref::new(raw)))
            }

            svec()
        }
    }
}
