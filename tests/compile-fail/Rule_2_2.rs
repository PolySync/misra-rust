#[deny(dead_code)]

const DEAD_CODE: i32 = 0;
//~^ ERROR 3:7: 3:16: constant `DEAD_CODE` is never used [dead_code]

fn main() {}
