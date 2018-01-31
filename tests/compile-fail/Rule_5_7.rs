#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

/// Struct definition shadows another `struct Deer`.
struct Deer { }

fn main() {
    let _: Deer = Deer { };
    {
        /// Struct definition shadows another `struct Deer`.
        struct Deer { }
        let _: Deer = Deer { };
    }
}
