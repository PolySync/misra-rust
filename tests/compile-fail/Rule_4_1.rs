#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let s1 = "\x41g";
    let _ = s1; // "Ag"
}
