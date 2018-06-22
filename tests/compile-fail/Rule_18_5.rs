fn nesting(p: &&&&&&[u8; 10]) {
    let _ = ****p;
}

fn main() {
    let a = [5; 10];
    nesting(&&&&&&a);
}
