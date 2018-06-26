
union UnionType {
    f1: i16,
    f2: i32,
}


fn main() {
    let x: UnionType = 0; //~ ERROR mismatched types
    let y: [[usize;2]; 3] = [0,1,2,3,4,5];
    //~^ ERROR mismatched types
}
