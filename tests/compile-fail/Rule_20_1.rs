struct MyStruct {
    a: u32,
}

fn func(_: MyStruct) {}

use std::fmt;
//~ ERROR Non-compliant - `include` directive preceeded by something other than macros or comments

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct{{ {} }}", self.a)
    }
}

fn main() {
    let s = MyStruct { a: 10 };
    func(s);
}
