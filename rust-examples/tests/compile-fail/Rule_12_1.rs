#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let x = 1;
    if x + 2 < 2 + 2 && x << 1 > 9 {
        //
    }
}