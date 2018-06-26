#[allow(unused_variables)]

fn main() {
    let mut x: Box<u8> = Box::new(8);
    //~^ ERROR Non-compliant - "pointer" is not const-qualified.
}
