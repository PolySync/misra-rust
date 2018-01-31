//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let x: bool = 3 as bool; //~ ERROR cannot cast as `bool`
    println!("{}", x);
}
