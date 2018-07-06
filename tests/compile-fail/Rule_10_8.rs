fn main() {
    let x: u16 = 1;
    let y: u16 = 2;
    let _: u32 = (x + y) as u32;
    //~^ ERROR Non-compliant - composite cast to wider type
}
