
#![feature(try_from)]

extern crate julia;

use std::convert::TryFrom;

use julia::api::{Julia, Value};

fn main() {
    let jl = Julia::new().unwrap();

    let sqrt = jl.base().function("sqrt").unwrap();
    let x = 3.0;
    let y = {
        let x = Value::from(x);
        sqrt.call1(&x).unwrap()
    };
    let y = f64::try_from(&y).unwrap();

    println!("sqrt({}) = {}", x, y);
}
