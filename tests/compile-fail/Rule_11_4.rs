#[derive(Debug)]
struct TypeA {
    f: f64,
}

fn main() {
    let i16_a: i16 = 1;
    let u32_b: u32 = &i16_a as &u16; //~ ERROR mismatched types
                                     //~^ ERROR non-primitive cast: `&i16` as `&u16`
}
