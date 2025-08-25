#[deny(dead_code)]

fn main() {
    type LocalType = i16; //~ ERROR type alias `LocalType` is never used [dead_code]
}
