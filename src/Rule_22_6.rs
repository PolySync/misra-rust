use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {

    let f2 = File::open("open.txt").unwrap();
    let mut contents = String::new();
    let mut buf_reader = BufReader::new(f2);

    buf_reader.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    drop(f2);

    buf_reader.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}