fn main() {
    let x: [i32] = //~ ERROR the size for values of type `[i32]` cannot be known at compilation time [E0277]
	 [0, 1]; //~ ERROR mismatched types

}
