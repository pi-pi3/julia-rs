
extern crate julia_sys;

use julia_sys::*;

use std::ffi::CStr;

unsafe fn cstr_as_string(mut string: *const i8) -> String {
    let mut ret = String::new();

    while *string != 0 {
        ret.push(*string as u8 as char);
        string = string.offset(1);
    }

    ret
}

fn main() {
    unsafe {
        jl_init();
        assert!(jl_is_initialized() != 0);

        let bytes = b"x\0";
        let string = CStr::from_bytes_with_nul(bytes).unwrap();
        jl_eval_string(string.as_ptr());

        let ex = jl_exception_occurred();
        let ex = jl_typeof_str(ex);
        let ex = cstr_as_string(ex);

        println!("ex = \"{}\"", ex);

        jl_atexit_hook(0);
    }
}
