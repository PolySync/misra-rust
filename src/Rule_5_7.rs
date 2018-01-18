#[derive(Debug)]
struct Deer {
    a: u16,
    b: u16,
}

fn main() {
    let d: Deer = Deer { a: 1, b: 2 };
    {
        #[derive(Debug)]
        struct Deer {
            c: u16,
            d: u16,
        }
        let e: Deer = Deer { c: 3, d: 4 };
        println!("{:?}", e);
    }
    println!("{:?}", d);
}
