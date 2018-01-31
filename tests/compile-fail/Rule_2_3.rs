#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    /// This type is never used.
    type LocalType = i16; //~ ERROR type alias is never used: `LocalType`
}
