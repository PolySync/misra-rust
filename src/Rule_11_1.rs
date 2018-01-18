fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: u32 = add_one as u32;
    println!("{}", x);
}
