fn recursive(depth: u64) {
    if depth == 2 {
        return
    }

    recursive(depth + 1);
}

fn main() {
    recursive(0);
}
