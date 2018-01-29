#![deny(clippy)]
#[deny(warnings)]

fn func() -> &'static i8 {
    let local_auto: i8 = 0;
    &local_auto
    //~^ ERROR `local_auto` does not live long enough
}

fn main() {
    func();
}
