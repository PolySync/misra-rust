//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let s1 = "\x41\x4g";
    //~^ ERROR invalid character in numeric character escape: g
    let _ = s1; // "A4g?"
}
