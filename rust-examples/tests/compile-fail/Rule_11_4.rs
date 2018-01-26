#![deny(clippy)]
#[deny(warnings)]

#[derive(Debug)]
struct TypeA {
    f: f64,
}

fn main() {
    let i16a: i16 = 1;
    let u32a: u32 = &i16a as &u16;

    println!("{}", u);
}
