fn func() -> &'static i8 {
    let local_auto: i8 = 0;
    &local_auto //~ ERROR cannot return reference to local variable `local_auto` [E0515]
}

fn main() {
    func();
}
