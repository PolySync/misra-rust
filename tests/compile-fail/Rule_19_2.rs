#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

union UnionA {
    f1: i16,
    f2: i32,
}

fn main() {
    let mut u = UnionA { f2: 0 };
    u.f1 = 3;
}
