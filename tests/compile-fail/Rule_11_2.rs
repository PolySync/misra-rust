#[derive(Debug)]
struct Type {
    f: f64,
}

fn main() {
    let t = Type { f: 3.14 };
    let r1: &Type = t as &Type;
    //~^ ERROR non-primitive cast: `Type` as `&Type`
    let r2: &Type = From::from(t);
    //~^ ERROR the trait bound `&Type: std::convert::From<Type>` is not satisfied
}
