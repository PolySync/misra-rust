//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

/// This function returnd a value.
fn func(para1: u16) -> u16 {
    para1
}

/// This function discards the value returned by `func`
fn discarded(para2: u16) {
    func(para2);
    //~^ ERROR Non-compliant - `func` return discarded
}

fn main() {
    discarded(1);
}
