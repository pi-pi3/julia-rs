
extern crate julia;

use julia::error::Error;
use julia::api::{Julia, ToJulia, Exception};

fn main() {
    let mut jl = Julia::new().unwrap();

    let x = 5.0.to_julia().unwrap();
    println!("typename(5.0) = {:?}", x.typename());

    let x = jl.eval_string("x = 5").unwrap();
    println!("typename(x = 5) = {:?}", x.typename());

    let x = jl.eval_string("x").unwrap();
    println!("typename(x) = {:?}", x.typename());

    let x = Exception::with_value(5.0.to_julia().unwrap()).unwrap();
    println!("typename(5.0) = {:?}", x.typename());

    let y = jl.eval_string("y");
    let y = match y {
        Ok(y) => y,
        Err(Error::UnhandledException(ex)) => ex.into_inner(),
        _ => panic!(),
    };
    println!("typename(y) = {:?}", y.typename());
}
