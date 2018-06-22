/// This function has a side effect.
fn increment(x: &mut u8) -> &mut u8 {
    *x += 1;
    x
}

/// This function does not have a side effect.
fn add(x: u8, y: u8) -> u8 {
    x + y
}

fn main() {
    let mut x: u8 = 0;
    let _ = add(*increment(&mut x), x);
    //~ ERROR Non-compliant - evaluation order effects result
}
