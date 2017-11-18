
#[macro_use]
extern crate julia;

use julia::api::Julia;

fn main() {
    let mut _jl = Julia::new();
    println!("{}", jlvec![].unwrap());
    println!("{}", jlvec![1].unwrap());
    println!("{}", jlvec![1, 2].unwrap());
    println!("{}", jlvec![1, 2, 3, 4].unwrap());
    println!("{}", jlvec![1; 8].unwrap());
}
