//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

#[derive(Debug)]
struct Once {
    a: i32,
}

fn main() {
    let _ = Once { a: 1, a: 2 };
    //~^ ERROR field `a` specified more than once
}
