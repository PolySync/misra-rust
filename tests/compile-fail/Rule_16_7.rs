fn main() {
    let i = true;

    match i as bool {
        //~^ ERROR you seem to be trying to match on a boolean expression
        false => {
            let _ = 1;
        }
        _ => {
            let _ = 2;
        }
    }
}
