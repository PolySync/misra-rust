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
    let tb1: TypeB = ta as TypeB;
    //~^ ERROR non-primitive cast: `TypeA` as `TypeB`
    let tb2: TypeB = From::from(ta);
    //~^ ERROR the trait bound `TypeB: std::convert::From<TypeA>` is not satisfied
}
