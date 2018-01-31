//! Rule

#![forbid(clippy_pedantic)]
#![forbid(clippy)]
#![forbid(warnings)]

fn return_v1(depth: u64) ->  u64 {
    if depth == 2 { //~ ERROR if may be missing an else clause
        return depth
    }
}

fn return_v2(depth: u64) ->  u64 {
    if depth == 2 {
        return //~ ERROR `return;` in a function whose return type is not `()`
    }

    depth
}

fn main() {
    return_v1(0);
    return_v2(0);
}
