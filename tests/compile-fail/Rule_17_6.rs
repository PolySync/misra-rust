fn main() {
    let data: [u32; static 4]; //~ ERROR  expected one of `move`, `use`, `|`, or `||`, found `4`
}
