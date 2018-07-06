#[allow(unused_labels)]
#[deny(unreachable_code)]

fn main() {
    'outer: loop {
        'inner: loop {
            break 'outer;
        }
        'unreachable: loop {
            //~^ ERROR unreachable expression
            break 'outer;
        }
    }
}
