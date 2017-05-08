extern crate libsmac;

use libsmac::lexer::block_tree;
use libsmac::lexer::{grab_smaragdine_lexer, lex_branch, flatten_branch};
use libsmac::parser::grab_smaragdine_parser;

fn main() {
    let data = r#"
r"hey"
'9'
1234
-1234
123.2
.212
true
false
    "#;
    let mut block_tree = block_tree::BlockTree::new(&data, 0);
    let indents = block_tree.collect_indents();
    let lexed_root = lex_branch(&block_tree.tree(&indents));
    let flat_root = flatten_branch(&lexed_root);
    println!("\n{:#?}", lexed_root);
    println!("\n{:#?}", flat_root);
    let parser = grab_smaragdine_parser(flat_root);
    for t in parser {
        println!("{:#?}", t)
    }
}
