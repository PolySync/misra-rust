#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

macro_rules! data { //~ ERROR unused macro definition
   () => (3;);
}

fn main() {
    //
}
