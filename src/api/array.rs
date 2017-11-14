
//! Module providing wrappers for iteratable sequences.

use std::slice;

use sys::*;
use error::Result;
use api::{Value, JlValue};

jlvalues! {
    pub struct Array(jl_array_t);
    pub struct ByteArray(jl_array_t);
    pub struct Svec(jl_svec_t);
}

impl Array {
    /// Returns the length of the Array.
    pub fn len(&self) -> Result<usize> {
        let len = unsafe { jl_array_len(self.lock()?) };
        Ok(len)
    }

    /// Checks if the Array is empty.
    pub fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) == 0
    }

    /// Constructs a Vec of Values from the Array.
    pub fn as_vec(&self) -> Result<Vec<Value>> {
        let len = self.len()?;
        let ptr = unsafe { jl_array_data(self.lock()?) as *mut *mut jl_value_t };
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        let vec = slice
            .iter()
            .map(|raw| unsafe { Value::new_unchecked(*raw) })
            .collect();
        Ok(vec)
    }

    /// Returns the value at a specified index.
    pub fn index(&self, idx: usize) -> Result<Value> {
        let raw = unsafe { jl_array_ptr_ref(self.lock()?, idx) };
        Value::new(raw)
    }

    /// Sets the value at a specified index.
    pub fn index_set(&self, idx: usize, x: &Value) -> Result<()> {
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

    /// Constructs a Vec of Values from the ByteArray.
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

    /// Constructs a Vec of Values from the Svec.
    pub fn as_vec(&self) -> Result<Vec<Value>> {
        let len = self.len()?;
        let ptr = unsafe { jl_svec_data(self.lock()?) };
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        let vec = slice
            .iter()
            .map(|raw| unsafe { Value::new_unchecked(*raw) })
            .collect();
        Ok(vec)
    }

    /// Returns the value at a specified index.
    pub fn index(&self, idx: usize) -> Result<Value> {
        let raw = unsafe { jl_svecref(self.lock()?, idx) };
        Value::new(raw)
    }

    /// Sets the value at a specified index.
    pub fn index_set(&self, idx: usize, x: &Value) -> Result<()> {
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
            use $crate::api::JlValue;
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let raw = unsafe { $crate::sys::jl_svec(0) };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec()
        }
    };
    [$elem:expr] => {
        {
            use $crate::api::JlValue;
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let elem = $crate::api::Value::from($elem).into_inner()?;
                let raw = unsafe {
                    $crate::sys::jl_svec1(elem as *mut _)
                };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec()
        }
    };
    [$elem1:expr, $elem2:expr] => {
        {
            use $crate::api::JlValue;
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let elem1 = $crate::api::Value::from($elem1).into_inner()?;
                let elem2 = $crate::api::Value::from($elem2).into_inner()?;
                let raw = unsafe {
                    $crate::sys::jl_svec2(elem1 as *mut _, elem2 as *mut _)
                };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec()
        }
    };
    [$( $elem:expr ),+] => {
        {
            use $crate::api::JlValue;
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
                let raw = unsafe {
                    $crate::sys::jl_svec(count,
                                         $(
                                             $crate::api::Value::from($elem).into_inner()?
                                         ),+
                                         )
                };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec(count)
        }
    };
    [$elem:expr; $n:expr] => {
        {
            use $crate::api::JlValue;
            fn svec() -> $crate::error::Result<$crate::api::Svec> {
                let elem = $crate::api::Value::from($elem).into_inner()?;
                let raw = unsafe {
                    $crate::sys::jl_svec_fill($n, elem)
                };
                jl_catch!();
                $crate::api::Svec::new(raw)
            }

            svec()
        }
    }
}
