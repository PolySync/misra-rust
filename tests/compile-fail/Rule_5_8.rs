pub static count: i32 = 1;

fn main() {
    let count: i32 = 1;
    //~^ ERROR let bindings cannot shadow statics
}
