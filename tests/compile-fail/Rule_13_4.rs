//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let a: u8;

    if a = 1 {
        //~^ ERROR mismatched types
    }
}
