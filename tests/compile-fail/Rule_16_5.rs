#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let i = 1;
    match i {
        0 => {},
        _ => {},
        1 => {}, //~ ERROR unreachable pattern
    }
}
