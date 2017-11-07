
use std::ffi::{CStr, CString};

use libc::c_char;

use error::Error;

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

pub trait IntoCString {
    fn into_cstring(self) -> CString;
}

pub trait TryIntoString {
    type Error;
    fn try_into_string(self) -> Result<String, Self::Error>;
}

impl IntoCString for CString {
    fn into_cstring(self) -> CString {
        self
    }
}

impl<'a> IntoCString for &'a CStr {
    fn into_cstring(self) -> CString {
        CStr::into_c_string(From::from(self))
    }
}

impl IntoCString for String {
    fn into_cstring(mut self) -> CString {
        self.push('\0');
        unsafe { CString::from_vec_unchecked(self.into_bytes()) }
    }
}

impl<'a> IntoCString for &'a String {
    fn into_cstring(self) -> CString {
        self.clone().into_cstring()
    }
}

impl<'a> IntoCString for &'a str {
    fn into_cstring(self) -> CString {
        let mut bytes = self.as_bytes().to_vec();
        bytes.push(0);
        unsafe { CString::from_vec_unchecked(bytes) }
    }
}

impl TryIntoString for *const c_char {
    type Error = Error;
    fn try_into_string(self) -> Result<String, Error> {
        if self.is_null() {
            Err(Error::NullPointer)
        } else {
            unsafe { CStr::from_ptr(self).try_into_string() }
        }
    }
}

impl<'a> TryIntoString for &'a CStr {
    type Error = Error;
    fn try_into_string(self) -> Result<String, Error> {
        self.to_owned().try_into_string()
    }
}

impl TryIntoString for CString {
    type Error = Error;
    fn try_into_string(self) -> Result<String, Error> {
        self.into_string().map_err(From::from)
    }
}

impl<'a> TryIntoString for &'a str {
    type Error = Error;
    fn try_into_string(self) -> Result<String, Error> {
        Ok(self.to_owned())
    }
}

impl TryIntoString for String {
    type Error = Error;
    fn try_into_string(self) -> Result<String, Error> {
        Ok(self)
    }
}
