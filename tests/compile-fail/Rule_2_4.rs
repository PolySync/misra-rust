#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    /// This enum is never used.
    enum State { //~ ERROR enum is never used: `State`
        /// Never used.
        SInit,
        /// Never used.
        SRun,
        /// Never used.
        SSleep,
    };
}
