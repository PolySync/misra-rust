#[deny(unreachable_code)]

fn main() {
    'outer: loop {
        'inner: loop {
            break 'outer;
        }
        'label1: loop {
            //~ ERROR unreachable statement
            println!("This label is unused!");
        }
        break;
    }
}
