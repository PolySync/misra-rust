#![deny(clippy)]
#[deny(warnings)]

fn main() {
    let a: i32 = 0;
    if (a < 10) && (a > 20) {
        println!("Never true!");
    }
}
