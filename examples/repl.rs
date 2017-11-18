
extern crate julia;

use std::io::{self, Write};

use julia::api::{Julia, Ref};

fn main() {
    let mut jl = Julia::new();

    let println = jl.base().function("println").unwrap();

    loop {
        let mut input = String::new();

        io::stdout().write_all(b">>> ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let ret = jl.eval_string(&input).unwrap_or_else(|_| {
            println!("invalid expression");
            Ref::nothing()
        });

        if !ret.is_nothing() {
            println.call(&[ret]).unwrap();
        }
    }
}
