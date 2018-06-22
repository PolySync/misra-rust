fn main() {
    let a1: [i32; 10] = [0; 10];
    let a2: [i32; 10] = [0; 10];

    let _ = &a1 - &a2;
    //~^ ERROR binary operation `-` cannot be applied to type `&[i32; 10]`
}
