// Enforce MISRA-C Rule 2.6 at compile time
#[deny(unreachable_code)]

fn main() {
    println!("This program contains an unused label declaration.");

    'outer: loop {
        'inner: loop {
            break 'outer;
        }
        'label1: loop {
            println!("This label is unused!");
        }
        break;
    }
}
