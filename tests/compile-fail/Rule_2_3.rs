#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    type LocalType = i16; //~ ERROR type alias is never used: `LocalType`
}
