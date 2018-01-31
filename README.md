
# MISRA-Rust
An investigation into what adhering to each MISRA-C rule looks like in Rust.
The intention is to decipher how much we "get for free" from the Rust compiler.

### Usage
To run tests use `cargo test`. Each rule corresponds to a source file in
`tests/compile-fail`. A "passing" test is a test that fails to compile,
indication that Rust enforces that MISRA rule.
