#[deny(unused_variables)]

fn has_unused_parameter(p1: &mut u16, unused: i16) {
    //~^ ERROR unused variable: `unused`
    *p1 = 0;
}

fn main() {
    let mut p1: u16 = 0;
    has_unused_parameter(&mut p1, 2);
}
