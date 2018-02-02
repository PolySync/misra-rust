//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

macro_rules! println { //~ ERROR Non-compliant - redefinition of macro name
   () => (3;);
}

fn main() {
    println!();
}
