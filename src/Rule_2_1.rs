// Enforce MISRA-C Rule 2.1 at compile time
#[deny(unreachable_code)]

fn main() {
    println!("This program contains unreachable code.");
    let res: u16;

    return;

    res = 3;
}
