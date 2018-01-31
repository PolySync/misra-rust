//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let unsigned: u32 = 1;
    let signed: i32 = 2;
    let _ = signed + unsigned; //~ ERROR mismatched types
    //~^ ERROR the trait bound `i32: std::ops::Add<u32>` is not satisfied
}
