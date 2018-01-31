//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let a: [u8; 10] = [0;10];
    let ptr = &a[5];

    (ptr + 5) = 0;
    //~^ ERROR invalid left-hand side expression
}
