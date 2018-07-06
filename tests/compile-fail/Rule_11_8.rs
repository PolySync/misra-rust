fn main() {
    let a = 1;
    *(&mut a) = 2;
    //~^ ERROR cannot borrow immutable local variable `a` as mutable
}
