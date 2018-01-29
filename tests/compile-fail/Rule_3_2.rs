#![deny(clippy)]
#[deny(warnings)]

fn main() {
    // \
    compile_error_if_not_commented(true);
    //~^ ERROR cannot find function `compile_error_if_not_commented` in this scope
}
