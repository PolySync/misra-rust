//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let data: [u32; static 4];
    //~^ ERROR expected expression, found keyword `static`
}
