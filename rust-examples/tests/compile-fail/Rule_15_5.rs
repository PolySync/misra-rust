#![deny(clippy)]
#[deny(warnings)]

fn main() {

    let x = 1;

    if x > 1 {
        return;
    }

    if x < 1 {
        return;
    }
}
