struct Deer {}

fn main() {
    let _: Deer = Deer {};
    {
        struct Deer {}
        //~^ ERROR Non-compliant - struct name shadows struct Deer in global scope
        let _: Deer = Deer {};
    }
}
