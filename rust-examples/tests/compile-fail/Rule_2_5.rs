#![deny(clippy)]
#[deny(warnings)]

macro_rules! data { //~ ERROR unused macro definition
   () => (3;);
}

fn main() {
    //
}
