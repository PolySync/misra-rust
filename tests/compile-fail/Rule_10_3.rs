#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let x: u32 = 2;
    let y: u16 = x; //~ ERROR mismatched types
}
