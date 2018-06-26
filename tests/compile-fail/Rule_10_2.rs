fn main() {
    let x: i16 = -2;
    let y = x - 'a';
    //~^ ERROR cannot subtract `char` from `i16`
}
