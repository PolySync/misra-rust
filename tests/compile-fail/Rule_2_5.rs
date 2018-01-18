#[deny(unused_macros)]

macro_rules! data {
    //~^ ERROR unused macro definition
    () => {
        3;
    };
}

fn main() {}
