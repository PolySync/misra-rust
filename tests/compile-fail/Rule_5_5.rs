#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

macro_rules! val {
   () => (3;);
}

fn main() {
    let val: i16 = 1;
    println!("{} -- {}", val, val!());
}
