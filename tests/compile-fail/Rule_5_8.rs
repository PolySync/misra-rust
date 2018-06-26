pub static count: i32 = 1;

fn main() {
    let count: i32 = 1;
    //~^ ERROR cannot be named the same as a static
}
