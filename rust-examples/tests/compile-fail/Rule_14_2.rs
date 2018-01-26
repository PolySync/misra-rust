#![deny(clippy)]
#[deny(warnings)]

fn main() {

    let mut bound: u32 = 100;

    for i in 0..bound {
        bound -= i;
    }

    println!("{}", bound);
}

