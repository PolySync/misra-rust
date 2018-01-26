#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let i = true;

    match i as bool {
        false => {},
        _ => {},
    }
}
