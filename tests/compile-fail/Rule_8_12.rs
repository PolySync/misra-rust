enum Uniqueness {
    red = 3,
    blue,
    green,
    yellow = 5, //~ ERROR enum already has `5`
}

fn main() {}
