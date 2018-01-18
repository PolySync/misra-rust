#[deny(dead_code)]

const DEAD_CODE: i32 = 0;
//~^ ERROR constant item is never used: `DEAD_CODE`

fn main() {}
