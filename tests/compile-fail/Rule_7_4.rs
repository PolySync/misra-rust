#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let mut literal = "string literal";
    literal[0] = 'S';
    //~^ ERROR the trait bound `str: std::ops::IndexMut<{integer}>` is not satisfied
    literal += ", modified?";
    //~^ ERROR binary assignment operation `+=` cannot be applied to type `&str`
}
