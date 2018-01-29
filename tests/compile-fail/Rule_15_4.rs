#![deny(clippy)]
#[deny(warnings)]

fn main() {
    'L1: for x in 0..5 {
        if x == 1 {
            break 'L1;
        }

        if x == 2 {
            break 'L1;
        }
    }
}
