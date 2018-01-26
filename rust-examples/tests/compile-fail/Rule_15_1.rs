#![deny(clippy)]
#[deny(warnings)]

fn main() {
    'L1: for _ in 0..5 {
        break 'L1;
    }
}
