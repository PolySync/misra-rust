//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let mut s = 10;
    let a: [u32; s];
    //~^ ERROR attempt to use a non-constant value in a constant
}
