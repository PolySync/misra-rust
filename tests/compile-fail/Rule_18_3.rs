#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let a1: [i32; 10] = [0;10];
    let a2: [i32; 10] = [0;10];

    let p1 = &a1;

    if p1 >= &a2 {
        //
    }
}
