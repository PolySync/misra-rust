#![deny(clippy)]
#[deny(warnings)]

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() {

    let mut f1 = OpenOptions::new()
    .read(true)
    .write(true)
    .append(true)
    .open("open.txt")
    .unwrap();

    f1.write_all(b"\n[data written by Rule_22_3]").unwrap();

    let f2 = File::open("open.txt").unwrap();
    let mut contents = String::new();


    let mut buf_reader = BufReader::new(f2);

    buf_reader.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}