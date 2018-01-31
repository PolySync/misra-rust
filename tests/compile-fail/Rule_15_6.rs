//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    loop
        let _ = 1;
        //~^ ERROR expected `{`, found `let`
}
