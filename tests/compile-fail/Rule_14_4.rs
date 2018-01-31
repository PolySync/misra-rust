//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let a: i32 = 0;
    if a {
        //~^ ERROR mismatched types
    }
}
