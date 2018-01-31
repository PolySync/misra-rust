//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let _: [i32] = [ 0, 1 ];
    //~^ ERROR mismatched types
}
