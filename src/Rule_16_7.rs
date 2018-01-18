#[deny(unreachable_patterns)]

fn main() {
    let i = true;

    match i as bool {
        false => {},
        _ => {},
    }
}
