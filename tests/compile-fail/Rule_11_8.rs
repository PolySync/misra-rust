fn main() {
    let a = 1;
    *(&mut a) = 2; //~ ERROR cannot borrow `a` as mutable, as it is not declared as mutable [E0596]
}
