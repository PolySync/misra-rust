fn main() {
    let i = 1;
    match i {
        _ => {} //~ ERROR Non-compliant - less that two clauses
    }
}
