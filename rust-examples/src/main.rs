#![feature(plugin)]
#![plugin(clippy)]

fn main(){
    let x = Some(1u8);
    match x {
        Some(y) => println!("{:?}", y),
        _ => ()
    }
}