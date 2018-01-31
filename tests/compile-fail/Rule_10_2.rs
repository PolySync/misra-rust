//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let x: i16 = -2;
    let _ = x - 'a';
    //~^ ERROR the trait bound `i16: std::ops::Sub<char>` is not satisfied
}
