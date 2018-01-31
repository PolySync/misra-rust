//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let i: i16 = 1;
    if true {
        let i: i16 = 0; //~ ERROR `i` is shadowed by `0`
        let _ = i;
    }
    let _ = i;
}