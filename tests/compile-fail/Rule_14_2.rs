#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {

    let mut bound: u32 = 100;

    for i in 0..bound {
        bound -= i;
        //~^ ERROR attempt to mutate range bound within loop; note that the range of the loop is unchanged
    }
}
