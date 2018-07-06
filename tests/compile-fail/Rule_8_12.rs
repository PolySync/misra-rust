enum Uniqueness {
    red = 3,
    blue,
    green,
    yellow = 5, //~ ERROR discriminant value `5` already exists
}

fn main() {}
