// Enforce MISRA-C Rule 2.4 at compile time
#[deny(dead_code)]

fn main() {
    println!("This program contains an unused tag declaration.");
    enum State {
        SInit,
        SRun,
        SSleep,
    };
}
