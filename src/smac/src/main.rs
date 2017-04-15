extern crate libsmac;

use libsmac::lexer::grab_smaragdine_lexer;

fn main() {
    let mut data = r#"
let x = 12
let f(y) = x + y
let b = true
let b? = b
let b!(val: bool) = b = val
let c = 0x12345678
let d = 0b10101010
let s  = "everything's escaped here \n\t\r means nothing"
let s? = 'non-escaped \'string\' \n\tliteral'
let char = '\n'
let not_char = "a"
let empty = ""
    "#.chars();

    let lexer = grab_smaragdine_lexer(&mut data);

    for t in lexer {
        println!("{}", t)
    }
}
