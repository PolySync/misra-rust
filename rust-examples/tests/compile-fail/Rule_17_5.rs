#![deny(clippy)]
#[deny(warnings)]

fn fn1(data: [u32; 3]) {
    unimplemented!()
}

fn main() {
    let data: [u32; 4];
    fn1(data);
}
