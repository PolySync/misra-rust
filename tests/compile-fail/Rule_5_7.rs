//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

/// Struct definition shadows another `struct Deer`.
struct Deer { }

fn main() {
    let _: Deer = Deer { };
    {
        /// Struct definition shadows another `struct Deer`.
        struct Deer { }
        //~^ ERROR Non-compliant - struct name shadows struct Deer
        let _: Deer = Deer { };
    }
}
