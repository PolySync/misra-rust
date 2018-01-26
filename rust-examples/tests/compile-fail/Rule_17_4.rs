#![deny(clippy)]
#[deny(warnings)]

fn return_v1(depth: u64) ->  u64 {
    if depth == 2 {
        return depth
    }
}

fn return_v2(depth: u64) ->  u64 {
    if depth == 2 {
        return
    }

    depth
}

fn main() {
    return_v1(0);
    return_v2(0);
}
