
use std::ffi::{CStr, CString};

use error::{Result, Error};

#[macro_export]
macro_rules! cstr {
    ( $( $s:expr ),*) => {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(
                concat!(
                    $( $s:expr ),*
                    '\0'
                ).as_bytes()
            )
        }
    }
}

pub trait AsCString {
    fn as_cstring(self) -> CString;
}

pub trait TryAsString {
    fn try_as_string(self) -> Result<String>;
}

impl AsCString for CString {
    fn as_cstring(self) -> CString {
        self
    }
}

impl<'a> AsCString for &'a CStr {
    fn as_cstring(self) -> CString {
        CStr::into_c_string(From::from(self))
    }
}

impl AsCString for String {
    fn as_cstring(mut self) -> CString {
        self.push('\0');
        unsafe { CString::from_vec_unchecked(self.into_bytes()) }
    }
}

impl<'a> AsCString for &'a String {
    fn as_cstring(self) -> CString {
        self.clone().as_cstring()
    }
}

impl<'a> AsCString for &'a str {
    fn as_cstring(self) -> CString {
        let mut bytes = self.as_bytes().to_vec();
        bytes.push(0);
        unsafe { CString::from_vec_unchecked(bytes) }
    }
}

// raw C string pointer
impl TryAsString for *const i8 {
    fn try_as_string(self) -> Result<String> {
        if self.is_null() {
            return Err(Error::NullPointer);
        }

        let mut raw = self as *const u8;
        let mut vec = vec![];

        unsafe {
            while *raw != 0 {
                vec.push(*raw);
                raw = raw.offset(1);
            }
        }

        String::from_utf8(vec).map_err(From::from)
    }
}
