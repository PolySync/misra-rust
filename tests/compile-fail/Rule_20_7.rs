//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

macro_rules! three {
   () => 3; //~ ERROR macro rhs must be delimited
}

fn main() {
    let _ = three!();
}
