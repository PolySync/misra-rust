#[deny(unreachable_patterns)]

fn main() {
    let i = 1;
    match i {
        0 => {},
        _ => {},
        1 => {},
    }
}
