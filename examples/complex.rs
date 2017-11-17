
extern crate julia;

use julia::api::{Julia, ToJulia, FromJulia, Function};

fn main() {
    let mut jl = Julia::new().unwrap();

    let result = jl.eval_string("f(x) = x * 2 - 1").unwrap();
    let f = Function(result);

    let x = 3.0.to_julia().unwrap();
    let y = f.call1(&x).unwrap();
    let y = f64::from_julia(&y).unwrap();

    assert!((y - (3.0 * 2.0 - 1.0)).abs() < std::f64::EPSILON);
    println!("f({}) = {}", 3.0, y);
}
