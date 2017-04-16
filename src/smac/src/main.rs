extern crate libsmac;

use libsmac::lexer::block_tree;
use libsmac::lexer::{grab_smaragdine_lexer, lex_branch, flatten_branch};

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
    "#.chars();

    let lexer = grab_smaragdine_lexer(&mut data);

    for t in lexer {
        println!("{}", t)
    }

    let mut data2 = r#"
outer_block
    let a = 1.12
    let b = r"raw string in block"
    inner_block
        let char = '\n'
    let c = 12.23
    "#;

    let mut block_tree = block_tree::BlockTree::new(&data2, 0);
    let indents        = block_tree.collect_indents();

    let lexed_root = lex_branch(&block_tree.tree(&indents));

    println!("\n{:#?}", lexed_root);
    println!("\n{:#?}", flatten_branch(&lexed_root))
}
