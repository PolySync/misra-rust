#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

/// This function modifes its parameter.
fn paramod(mut para: u16) -> u16 {
    para += 1;
    para
}

fn main() {
    paramod(1);
}
