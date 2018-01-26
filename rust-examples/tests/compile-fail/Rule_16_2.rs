#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let x = 0..1;

    for i in x {
        match i {
            0 => {
                1 => { /* Malformed match. */ }
            },
            _ => { /*'default' case.*/ }
        }
    }
}
