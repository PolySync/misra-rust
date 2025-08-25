#[allow(unused_assignments, unused_variables)]

fn main() {
    let x: u16;
    return;
    x = 3; //~ ERROR unreachable statement [unreachable_code]
}
		