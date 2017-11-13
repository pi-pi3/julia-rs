
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
    pub fn len(&self) -> Result<usize> {
        let len = unsafe { jl_array_len(self.lock()?) };
        Ok(len)
    }

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

    pub fn index(&self, idx: usize) -> Result<Value> {
        let raw = unsafe { jl_array_ptr_ref(self.lock()?, idx) };
        Value::new(raw)
    }

    pub fn index_set(&self, idx: usize, x: &Value) -> Result<()> {
        unsafe { jl_array_ptr_set(self.lock()?, idx, x.lock()?); }
        Ok(())
    }
}

impl ByteArray {
    pub fn len(&self) -> Result<usize> {
        let len = unsafe { jl_array_len(self.lock()?) };
        Ok(len)
    }

    pub fn as_slice(&self) -> Result<&[u8]> {
        let len = self.len()?;
        let ptr = unsafe { jl_array_data(self.lock()?) as *mut u8 };
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        Ok((slice))
    }

    pub fn as_vec(&self) -> Result<Vec<u8>> {
        self.as_slice().map(|s| s.to_vec())
    }

    pub fn index(&self, idx: usize) -> Result<u8> {
        let byte = unsafe { jl_array_uint8_ref(self.lock()?, idx) };
        Ok(byte)
    }

    pub fn index_set(&self, idx: usize, x: u8) -> Result<()> {
        unsafe { jl_array_uint8_set(self.lock()?, idx, x); }
        Ok(())
    }
}

impl Svec {
    pub fn len(&self) -> Result<usize> {
        let len = unsafe { jl_svec_len(self.lock()?) };
        Ok(len)
    }

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

    pub fn index(&self, idx: usize) -> Result<Value> {
        let raw = unsafe { jl_svecref(self.lock()?, idx) };
        Value::new(raw)
    }

    pub fn index_set(&self, idx: usize, x: &Value) -> Result<()> {
        unsafe { jl_svecset(self.lock()?, idx, x.lock()?); }
        Ok(())
    }
}

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
