//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn fn1(data: [u32; 3]) {
    //
}

fn main() {
    let data: [u32; 4];
    fn1(data);
    //~^ ERROR mismatched types
}
