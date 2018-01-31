//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let a: [u32];
    //~^ ERROR the trait bound `[u32]: std::marker::Sized` is not satisfied
}
