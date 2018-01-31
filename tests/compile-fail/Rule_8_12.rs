//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

enum Uniqueness {
    red = 3,
    blue,
    green,
    yellow = 5, //~ ERROR discriminant value `5isize` already exists
}

fn main() {
    //
}
