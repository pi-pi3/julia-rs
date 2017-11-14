
# Contributing to julia-rs

Any contribution is welcome.

- Did you spot an error?  
    Fork and make a pull request.
- Is there an overflow or memory leak happening anywhere?  
    Fork and make a pull request.
- Did you spot a typo?  
    Fork and make a pull request.
- Is there any missing documentation?  
    Fork and make a pull request.

However before requesting a pull request, you are expected to validate your code
using the following steps.

1. `cargo clean && cargo update && cargo build`  
2. `cargo clippy` and fix any lints.  (You'll need `cargo-clippy` installed.)
    If you think a lint warning should be ignored, explain why.
3. `rustfmt`

Additionally, you are expected to test any new code or any code you've
modified.  
Currently for reasons unknown, test builds made with `cargo test` will fail when
calling any extern Julia function, so tests might have to be written as
**[examples](/examples)**.
