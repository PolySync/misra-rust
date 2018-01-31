#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let res: u16;
    return;
    res = 3; //~ ERROR unreachable statement
}
