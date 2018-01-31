#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    enum State { //~ ERROR enum is never used: `State`
        SInit,
        SRun,
        SSleep,
    };
}
