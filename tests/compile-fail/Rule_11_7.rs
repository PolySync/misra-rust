fn main() {
    let p: &i16;
    let _ = p as f32;
    //~^ ERROR casting `&i16` as `f32` is invalid
}
