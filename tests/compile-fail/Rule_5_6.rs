#[allow(dead_code)]

fn main() {
    type U8 = bool;
    {
        type U8 = u8;
        //~^ ERROR Non-compliant - type name shadows U8
    }
}
