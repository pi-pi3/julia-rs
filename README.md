
# julia-rs

[![crates.io](https://img.shields.io/crates/v/julia.svg)](https://crates.io/crates/julia)
[![Docs](https://docs.rs/julia/badge.svg)](https://docs.rs/julia)

Safe and idiomatic Julia bindings for Rust

```
[dependencies]
julia = "0.1"
...
```

**[CONTRIBUTING.md](/CONTRIBUTING.md)**

# REPL

As an example application, an interactive Julia REPL written in 100% safe Rust
is included. See its source at **[src/main.rs](/src/main.rs)**, build with
`cargo build` and run with `cargo run`. See `julia-rs -h` or `cargo run -- -h`
for help.

# Example

See **[examples](/examples)** for more examples.

```rust
fn main() {
    use julia::api::{Julia, Value};

    let mut jl = Julia::new().unwrap();
    jl.eval_string("println(\"Hello, Julia!\")").unwrap();
    // Hello, Julia!

    let sqrt = jl.base().function("sqrt").unwrap();

    let boxed_x = Value::from(1337.0);
    let boxed_sqrt_x = sqrt.call1(&boxed_x).unwrap();

    let sqrt_x = f64::try_from(boxed_sqrt_x).unwrap();
    println!("{}", sqrt_x);
    // 36.565010597564445
}
```

# TODO

## pre 0.3
- Safe wrappers for all remaining important functions.  
  See **[ROADMAP.md](/ROADMAP.md)** for details.

## pre 0.4
- Fix any and all memory leaks and overflows at the ffi boundary.

## pre 0.5
- Standardize.
- Test.
- Add more thorough examples.

## pre 0.6
- Standardize documentation.
- Document everything thouroughly.

# License

julia-rs is licensed under the zlib/libpng license. See
**[LICENSE](/LICENSE)** or
[zlib.net/zlib\_license.html](http://www.zlib.net/zlib_license.html)
for details.
