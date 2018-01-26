#![deny(clippy)]
#[deny(warnings)]

#[derive(Debug)]
struct TypeA {
    f: f64,
}

#[derive(Debug)]
struct TypeB {
    f: f64,
}

fn main() {
    let ta = TypeA { f: 3.14 };
    let tb1: TypeB = ta as TypeB; // error
    let tb2: TypeB = From::from(ta); // error

    println!("{:?} -- {:?}", tb1, tb2);
}
