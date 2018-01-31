//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

/// This constant name is non-compliant, it's too close to the later 'abc'.
const ABC: i32 = 0;

fn main() {

    let abc: i32 = 1;
    //~^ ERROR Non-compliant - variable name shadows ABC
    let _ = abc;
    let _ = ABC;
}
