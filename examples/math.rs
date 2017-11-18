
extern crate julia;

use julia::api::{Julia, ToJulia, FromJulia};

fn main() {
    let jl = Julia::new();

    let sqrt = jl.base().function("sqrt").unwrap();
    let x = 3.0;
    let y = {
        sqrt.call1(&x.to_julia().unwrap()).unwrap()
    };
    let y = f64::from_julia(&y).unwrap();

    println!("sqrt({}) = {}", x, y);
}
