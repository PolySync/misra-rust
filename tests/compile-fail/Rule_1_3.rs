#![deny(unsafe_code)]

fn main() {
    let _x = unsafe { *(&0 as *const i32) };
    //~^ ERROR usage of an `unsafe` block
}
