
fn func() -> &'static i8 {
    let local_auto: i8;
    &local_auto
}

fn main() {
    func();
}
