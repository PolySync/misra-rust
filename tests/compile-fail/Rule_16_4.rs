fn main() {
    let i = 1;
    match i {
        //~ ERROR non-exhaustive patterns: `_` not covered
        0 => {}
    }
}
