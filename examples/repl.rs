
extern crate julia;

use std::io::{self, Write};

use julia::{Julia, Value, Function};
use julia::value::JlValue;

fn main() {
    let mut jl = Julia::new();

    let println = jl.eval_string("println").unwrap(); // TODO: get_function
    let println = Function::from_value(println).unwrap();

    loop {
        let mut input = String::new();

        io::stdout().write_all(b">>> ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let ret = jl.eval_string(&input).unwrap_or_else(|_| {
            println!("invalid expression");
            Value::nothing()
        });

        if !ret.is_nothing() {
            println.call(&[ret]).unwrap();
        }
    }
}
