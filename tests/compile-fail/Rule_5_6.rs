#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    /// Type shadows another `type U8`.
    type U8 = bool;
    {
        /// Type shadows another `type U8`.
        type U8 = u8;
        let _: U8 = 1;
    }

    let _: U8 = false;
}
