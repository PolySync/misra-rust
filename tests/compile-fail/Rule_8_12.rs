enum Uniqueness { //~ ERROR discriminant value `5` assigned more than once [E0081]
    RED = 3,
    BLUE,
    GREEN,
    YELLOW = 5,
}

fn main() {}
