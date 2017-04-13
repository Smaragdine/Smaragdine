extern crate libsmac;

use libsmac::lexer;
use lexer::matcher::{
    Whitespace, IntLiteral
};

fn main() {
    let mut data = r#"
1 2 3 4 5 6 7 8 9 10 420
    "#.chars();

    let tokenizer = lexer::Tokenizer::new(&mut data);
    let mut lexer = lexer::Lexer::new(tokenizer);

    let whitespace  = Whitespace {};
    let int_literal = IntLiteral {};

    lexer.matchers_mut().push(Box::new(whitespace));
    lexer.matchers_mut().push(Box::new(int_literal));

    for t in lexer {
        println!("{}", t)
    }
}
