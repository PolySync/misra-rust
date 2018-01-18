/// This function has a side effect.
fn not(x: &mut bool) -> &mut bool {
    *x = !*x;
    x
}

fn main() {
    let mut x: bool = true;

    if *not(&mut x) || *not(&mut x) {
        //~^ ERROR Non-compliant - right hand operand contains persistent side-effects
    }
}
