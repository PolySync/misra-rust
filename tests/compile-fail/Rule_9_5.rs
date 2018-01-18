fn main() {
    let x: [i32] = [0, 1]; //~ ERROR mismatched types
                           //~^ ERROR the size for value values of type `[i32]` cannot be known at compilation time
}
