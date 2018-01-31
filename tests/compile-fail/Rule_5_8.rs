#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

const count: i32 = 0;

fn main() {
    let count: i32 = 1;
    //~^ ERROR refutable pattern in local binding: `_` not covered
    let _ = count;
}
