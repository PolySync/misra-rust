
fn increment(x: &mut u8) -> &mut u8 {
    *x += 1;
    x
}

fn add( x: u8, y: u8 ) -> u8 {
    x + y
}

fn main() {
    let mut x: u8 = 0;
    // println!("{:?}", add(x, *increment(&mut x))); // different result!
    println!("{:?}", add(*increment(&mut x), x));
}
