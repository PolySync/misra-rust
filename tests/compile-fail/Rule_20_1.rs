#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

/// This struct definition preceeds the `use` statement.
struct MyStruct {
    /// This field is arbitrary.
    a: u32,
}

/// This function definition preceeds the `use` statement.
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
