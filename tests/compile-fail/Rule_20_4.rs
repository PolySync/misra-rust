#![deny(clippy)]
#[deny(warnings)]

macro_rules! while { //~ ERROR expected identifier, found keyword `while`
   () => (3;);
}
//~^ ERROR unused macro definition

fn main() {
    let _ = while!();
    //~^ ERROR expected one of `.`, `?`, `{`, or an operator, found `;`
}
