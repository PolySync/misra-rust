#![deny(clippy)]
#[deny(warnings)]

fn paramod(mut para: u16) -> u16 {
    para = para + 1;

    return para;
}

fn main() {
    paramod(1);
}
