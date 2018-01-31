//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn main() {
    'outer: loop {
        'inner: loop {
            break 'outer;
        }
        'label1: loop { //~ ERROR unreachable statement
            println!("This label is unused!");
        }
        break;
    }
}
