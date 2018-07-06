fn main() {
    let data: [u32; static 4];
    //~^ ERROR expected one of `move`, `|`, or `||`, found `4`
}
