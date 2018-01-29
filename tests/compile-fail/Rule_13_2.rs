#![deny(clippy)]
#[deny(warnings)]

fn increment(x: &mut u8) -> &mut u8 {
    *x += 1;
    x
}

fn add( x: u8, y: u8 ) -> u8 {
    x + y
}

fn main() {
    let mut x: u8 = 0;
    let _ = add(*increment(&mut x), x);
}
