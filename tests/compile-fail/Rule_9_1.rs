//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let x: u16;
    let _ = x + 1;
    //~^ ERROR use of possibly uninitialized variable: `x`
}
