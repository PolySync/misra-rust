#[deny(unreachable_code)]

fn main() {
    let x: u16;
    return;
    x = 3; //~ ERROR unreachable statement
}
