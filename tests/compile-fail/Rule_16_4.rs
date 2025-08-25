fn main() {
    let i = 1;
    match i { //~ ERROR non-exhaustive patterns: `i32::MIN..=-1_i32` and `1_i32..=i32::MAX` not covered [E0004]
        0 => {}
    }
}
