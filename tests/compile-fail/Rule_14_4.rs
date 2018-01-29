#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let a: i32 = 0;
    if a {
        //~^ ERROR mismatched types
    }
}
