macro_rules! double_it {
    ($e:expr) => {
        $e * $e
    };
}

fn main() {
    let _ = double_it!(
        #[cfg(feature = 10)]
        10
        #[cfg(feature = 100)]
        //~^ ERROR no rules expected the token `#`
        100
        );
}
