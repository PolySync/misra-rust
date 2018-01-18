#[derive(Debug)]
struct Type {
    f: f64,
}

fn main() {
    let t = Type { f: 3.14 };
    let r1: &Type = t as &Type; // error
    let r2: &Type = From::from(t); // error

    println!("{:?} -- {:?}", r1, r2);
}
