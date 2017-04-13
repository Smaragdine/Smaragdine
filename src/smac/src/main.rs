extern crate libsmac;

use libsmac::lexer;
use lexer::matcher::{
    Whitespace, IntLiteral, Symbol,
};

fn main() {
    let mut data = r#"
1 2 3
(1 2)
    "#.chars();

    let tokenizer = lexer::Tokenizer::new(&mut data);
    let mut lexer = lexer::Lexer::new(tokenizer);

    let symbols = vec![
        "(".to_string(),
        ")".to_string(),
    ];

    let symbol      = Symbol::new(symbols);
    let whitespace  = Whitespace {};
    let int_literal = IntLiteral {};

    lexer.matchers_mut().push(Box::new(whitespace));
    lexer.matchers_mut().push(Box::new(int_literal));
    lexer.matchers_mut().push(Box::new(symbol));

    for t in lexer {
        println!("{}", t)
    }
}
