fn main() {
    let mut bound: u32 = 100;

    for i in 0..bound {
        bound -= i;
        //~^ ERROR Non-compliant - attempt to mutate range bound within loop
    }
}
