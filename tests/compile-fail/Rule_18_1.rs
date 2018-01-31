//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let c: [i32; 2] = [0; 2];
    let mut p = &c[3];

    p += 1;
    //~^ ERROR binary assignment operation `+=` cannot be applied to type `&i32`
}
