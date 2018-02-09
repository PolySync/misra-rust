//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    let x = 5;
    let raw = &x as *const i32;
    let _ = unsafe { *raw };
    //~^ ERROR keyword 'unsafe' disallowed
}