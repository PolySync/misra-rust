#[deny(unused_mut)]

fn main() {
    let mut x: Box<u8> = Box::new(8);

    println!("using non-mutably: {}", x);
}
