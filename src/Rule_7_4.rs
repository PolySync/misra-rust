
fn main() {
    let mut literal = "string literal";
    literal[0] = 'S'; // error
    literal += ", modified?"; // error
    println!("{}", literal);
}
