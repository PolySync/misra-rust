fn main() {
    type U8 = bool;
    {
        type U8 = u8;
        let val: U8 = 1;
        println!("{}", val);
    }

    let val: U8 = false;

    println!("{}", val);
}
