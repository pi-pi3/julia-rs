
extern crate julia;

use julia::api::Julia;

fn main() {
    let mut jl = Julia::new();
    let result = jl.eval_string("println(\"Hello, world!\")");
    assert!(result.is_ok());
}
