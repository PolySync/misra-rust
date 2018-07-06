# MISRA-Rust

## Overview

The MISRA-Rust project represents an investigation into what adhering to each MISRA-C rule looks
like in Rust. The intention is to decipher how much we "get for free" from the Rust compiler.

Because of the proprietary nature of the MISRA-C specificaion, the description of each rule has
been omitted.

## Usage

Each rule corresponds to a source file in `tests/compile-fail`. A "passing" test is a test that
fails to compile. Passing tests can indicate that Rust enforces a specific MISRA-C rule or that
the rule does have a Rust equivalent.

```bash
cargo test
```

## Issues

We certainly want to hear if you think we've missed something or misrepresented a rule. Please raise
an issue!

# License

© 2018, PolySync Technologies, Inc.

* Shea Newton [email](mailto:snewton@polysync.io)

Please see the [LICENSE](./LICENSE) file for more details