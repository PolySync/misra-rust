#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

macro_rules! println {
   () => (3;);
}

fn main() {
    println!();
}
