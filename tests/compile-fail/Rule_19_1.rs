#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

union UnionA {
    f1: i16,
    f2: i32,
}

fn main() {
    let u = UnionA { f2: 0 };
    u.f1 = u.f2;
    //~^ ERROR mismatched types
}
