fn main() {
    let a1: [i32; 10] = [0; 10];
    let a2: [i32; 10] = [0; 10];

    let _ = &a1 - &a2; //~ ERROR cannot subtract `&[i32; 10]` from `&[i32; 10]` [E0369]
}
