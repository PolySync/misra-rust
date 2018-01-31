#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
#[cfg(custom_check = 10)]
    println!("The above is legal.");
}
