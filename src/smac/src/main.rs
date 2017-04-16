extern crate libsmac;

use libsmac::lexer::block_tree;
use libsmac::lexer::{grab_smaragdine_lexer, lex_branch, flatten_branch};
use libsmac::parser::grab_smaragdine_parser;

fn main() {
    let mut data = r#"
let x = 12
let f(y) = x + y
let b = true
let b? = b
let b!(val: bool) = b = val
let c = 0x12345678
let d = 0b10101010
let s  = r"everything's escaped here \n\t\r means nothing"
let s? = "non-escaped \"string\" \n\tliteral"
let char = '\n'
let not_char = "a"
let empty = ""
let float = .42
let f = 0.1337
let g = true || false
    "#.chars();

    let lexer = grab_smaragdine_lexer(&mut data);

    for t in lexer {
        println!("{}", t)
    }

    let data2 = r#"
r"hey"
'9'
123.2
.212
true
false
    "#;

    let mut block_tree = block_tree::BlockTree::new(&data2, 0);
    let indents        = block_tree.collect_indents();

    let lexed_root = lex_branch(&block_tree.tree(&indents));

    let flat_root = flatten_branch(&lexed_root);

    println!("\n{:#?}", lexed_root);
    println!("\n{:#?}", flat_root);

    let parser = grab_smaragdine_parser(flat_root);

    for t in parser {
        println!("{:#?}", t)
    }
}
