// Enforce MISRA-C Rule 2.7 at compile time
#[deny(unused_variables)]

fn has_unused_parameter( p1: &mut u16, unused: i16 ) {
    *p1 = 42;
}
fn main() {
    println!("This program contains an unused function parameter.");
    let mut p1: u16 = 0;
    has_unused_parameter(&mut p1, 2);
}
