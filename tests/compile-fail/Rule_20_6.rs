#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

macro_rules! print_it {
   ($e:expr) => (println!("{:?}", $e));
}

fn main() {
    print_it!(
        if cfg!(feature = "hi") {
            "hi"
        } else {
            "hello"
        }
    );
}

