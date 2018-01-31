//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let u8a: u8 = 0;
    let _ = u8a - 10;
    //~^ WARNING attempt to subtract with overflow
}
