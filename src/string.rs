
use std::ffi::{CStr, CString};

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
