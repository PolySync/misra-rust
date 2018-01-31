#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

/// This function's parameter's nested reference level is non-compliant.
fn nesting(p: &&&[u8;10]) {
    let _ = ***p;
}

fn main() {
    let a = [5;10];
    nesting(&&&a);
}
