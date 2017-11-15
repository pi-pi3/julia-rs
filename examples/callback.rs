
#[macro_use]
extern crate julia;

use julia::api::{Julia, Float64};

extern_jl! {
    extern "Julia" libsquare :: Square {
        pub fn square(x: Float64) -> Float64 {
            x * x
        }
    }
}

fn main() {
    let mut jl = Julia::new().unwrap();

    let sqr = Square::new();
    sqr.decl(&mut jl).unwrap();

    jl.eval_string("square").unwrap();
}
