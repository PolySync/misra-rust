#[allow(unused_variables)]

fn main() {
    let compliant_unsigned: u32 = 1u32;
    let unsigned: u32 = 1;
    //~^ ERROR Non-compliant - suffix specifying unsigned type required on integer constants.
}