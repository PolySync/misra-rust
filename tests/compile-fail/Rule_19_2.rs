//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

/// The use of the `union` keyword is non-compliant.
union UnionA { //~ ERROR Non-compliant - use of `union` keyword
    /// 'Field 1', distinct type.
    f1: i16,
    /// 'Field 2', distinct type.
    f2: i32,
}

fn main() {
    let mut u = UnionA { f2: 0 };
    u.f1 = 3;
}
