#![deny(clippy)]
#[deny(warnings)]

fn not(x: &mut bool) -> &mut bool {
    *x = !*x;
    x
}

fn main() {
    let mut x: bool = true;

    if *not(&mut x) || *not(&mut x) {
        //
    }
}
