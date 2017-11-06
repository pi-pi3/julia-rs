
extern crate julia;

use julia::{Julia, Value};
use julia::value::JlValue;
use julia::exception::Exception;
use julia::error::Error;

fn main() {
    let mut jl = Julia::new().unwrap();

    let x = Value::from(5.0);
    println!("typename(5.0) = {:?}", x.typename());

    let x = jl.eval_string("x = 5").unwrap();
    println!("typename(x = 5) = {:?}", x.typename());

    let x = jl.eval_string("x").unwrap();
    println!("typename(x) = {:?}", x.typename());

    let x = Exception::with_value(Value::from(5.0)).unwrap();
    println!("typename(5.0) = {:?}", x.typename());

    let y = jl.eval_string("y");
    let y = match y {
        Ok(y) => y,
        Err(Error::UnhandledException(ex)) => ex.into_inner(),
        _ => panic!(),
    };
    println!("typename(y) = {:?}", y.typename());
}
