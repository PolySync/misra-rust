#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let mut x: u8 = 0;
    let _: [&u8; 2] = [ &mut x, &mut x];
    //~^ ERROR cannot borrow `x` as mutable more than once at a time
}
