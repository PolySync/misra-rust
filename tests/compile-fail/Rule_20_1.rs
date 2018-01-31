#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

struct MyStruct {
    a: u32,
}

fn func(_: MyStruct) {
    //
}

use std::fmt;

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct{{ {} }}", self.a)
    }
}

fn main() {
    let s = MyStruct{ a: 10 };
    func(s);
}
