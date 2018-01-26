#![deny(clippy)]
#[deny(warnings)]

fn main(){
    let x = Some(1u8);
    match x { //~ ERROR you seem to be trying to use match for destructuring a single type. Consider using `if let`, #[warn(single_match)] on by default
        Some(y) => println!("{:?}", y),
        _ => ()
    }
}

// fn main() {
//     let res: u16;
//     return;
//     res = 3; //~ ERROR unreachable statement
// }
