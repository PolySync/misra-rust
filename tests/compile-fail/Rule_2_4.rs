#[deny(dead_code)]

fn main() {
    enum State {
        //~^ ERROR enum `State` is never used [dead_code]
        SInit,
        SRun,
        SSleep,
    }}
