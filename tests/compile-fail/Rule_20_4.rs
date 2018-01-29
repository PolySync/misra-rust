#![deny(clippy)]
#[deny(warnings)]

macro_rules! while { //~ ERROR expected identifier, found keyword `while`
//~^ ERROR unused macro definition
   () => (3;);
}

fn main() {
    let _ = while!();
    //~^ ERROR expected one of `.`, `?`, `{`, or an operator, found `;`
}
