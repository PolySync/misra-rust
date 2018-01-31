#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    let x = 0;

    /* End comment marker omitted. //~ ERROR unterminated block comment
        Perform_Critical_Safety_Function(x)
    /* Non-compliant comment. */

    println!("This program contains comments in a comment.");
}
