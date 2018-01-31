#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let x = 0..1;

    for i in x {
        match i {
            0 => {
                1 => { /* Malformed match. */ }  //~ ERROR mismatched types
                //~^ ERROR expected one of `.`, `;`, `?`, `}`, or an operator, found `=>`
            },
            _ => { /*'default' case.*/ }
        }
    }
}
