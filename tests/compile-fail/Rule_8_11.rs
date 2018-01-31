#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

static arr: [i32] = []; //~ ERROR mismatched types
//~^ ERROR the trait bound `[i32]: std::marker::Sized` is not satisfied

fn main() {
    //
}
