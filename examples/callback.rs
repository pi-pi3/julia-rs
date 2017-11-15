
#[macro_use]
extern crate julia;

use julia::api::Julia;
use julia::api::primitive::*;

extern_jl! {
    extern "Julia" libsquare :: LibSquare {
        pub fn square(x: Float64) -> Float64 {
            x * x
        }
    }
}

#[no_mangle]
pub extern "C" fn libsquare_decl() {
    // if Julia was created with Julia::new_unchecked, it won't trigger the
    // at_exit hooks.
    let mut jl = unsafe { Julia::new_unchecked() };

    let sqr = LibSquare::new();
    sqr.decl(&mut jl).unwrap();
}

// In Julia:
/*

dlname = "libsquare"
push!(Libdl.DL_LOAD_PATH, "./") 
libsquare = Libdl.dlopen(dlname)

ccall((:square_decl, :libsquare), Void, ())

println("square(4.0) = ", square(4.0))

 */

// This is just to make this compilable as a binary.
pub fn main() {}
