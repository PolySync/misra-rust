#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let count: i32 = 0o52;
    println!("{}", count);
}
