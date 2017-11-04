
#![feature(try_from)]

extern crate julia;

use std::convert::TryFrom;

use julia::{Julia, Value, Function};
use julia::value::JlValue;

fn main() {
    let mut jl = Julia::new();

    let result = jl.eval_string("f(x) = x * 2 - 1").unwrap();
    let f = Function::from_value(result).unwrap();

    let x = Value::from(3.0);
    let y = f.call1(&x).unwrap();
    let y = f64::try_from(&y).unwrap();

    assert!((y - (3.0 * 2.0 - 1.0)).abs() < std::f64::EPSILON);
    println!("f({}) = {}", 3.0, y);
}
