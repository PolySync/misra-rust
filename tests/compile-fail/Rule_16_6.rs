#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let i = 1;
    match i {
        _ => {},
    }
}
