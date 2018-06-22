/// This function modifes its parameter.
fn paramod(mut para: u16) -> u16 {
    para += 1; //~ ERROR parameter modified without persistent effect
    let _ = para;
    1
}

fn main() {
    paramod(1);
}
