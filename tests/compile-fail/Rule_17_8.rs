#![deny(clippy)]
#[deny(warnings)]

fn paramod(mut para: u16) -> u16 {
    para += 1;

    para
}

fn main() {
    paramod(1);
}
