#![deny(clippy)]
#[deny(warnings)]

fn main() {
    enum State { //~ ERROR enum is never used: `State`
        SInit,
        SRun,
        SSleep,
    };
}
