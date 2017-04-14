extern crate libsmac;

use libsmac::lexer::block_tree;
use libsmac::lexer::grab_smaragdine_lexer;

fn main() {
    let mut data = r#"
let x = 12
let f(y) = x + y
let b = true
let b? = b
let b!(val: bool) = b = val

outer_block
    let x = 12
    let f(y) = x + y

    inner_block
        let b = true
        let b? = b

    let b!(val: bool) = b = val

    "#.chars();

    let mut block_tree = block_tree::BlockTree::new(data.as_str(), 0);
    let indents        = &block_tree.collect_indents();

    println!("{:#?}", block_tree.tree(indents));

    let lexer = grab_smaragdine_lexer(&mut data);

    for t in lexer {
        println!("{}", t)
    }
}
