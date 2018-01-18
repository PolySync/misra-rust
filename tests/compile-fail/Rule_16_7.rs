fn main() {
    let i = true;

    match i as bool {
        //~^ ERROR Non-compliant - match on a boolean expression
        false => {
            let _ = 1;
        }
        _ => {
            let _ = 2;
        }
    }
}
