macro_rules! val {
    () => {
        3;
    };
}

fn main() {
    let val: i16 = 1;
    let _ = val;
    let _ = val!();
}
