use lexer::Tokenizer;
use lexer::token::{Token, TokenType, TokenPosition};

pub trait Matcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token>;
}

pub struct Whitespace {}

impl Matcher for Whitespace {
    fn try_match(
        &self,
        tokenizer: &mut Tokenizer,
    ) -> Option<Token> {
        let mut found = false;

        while !tokenizer.end() && " \n\r".contains(*tokenizer.peek().unwrap()) {
            found = true;
            tokenizer.next();
        }

        if found {
            return Some(
                Token::new(TokenType::Whitespace, TokenPosition::new(0, 0), "".to_string()),
            )
        }

        None
    }
}

pub struct IntLiteral {}

impl Matcher for IntLiteral {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut integer = "".to_owned();

        let mut start: usize = 0;

        while !tokenizer.end() && "0123456789".contains(*tokenizer.peek().unwrap()) {

            if integer.is_empty() {
                start = *tokenizer.index();
            }

            integer.push(tokenizer.next().unwrap())
        }

        if !integer.is_empty() {
            return Some(
                Token::new(
                    TokenType::IntLiteral,
                    TokenPosition::new(start, *tokenizer.index()),
                    integer,
                )
            )
        }

        None
    }
}

pub struct Symbol {
    symbols: Vec<String>,
}

impl Symbol {
    pub fn new(symbols: Vec<String>) -> Symbol {
        Symbol {
            symbols: symbols
        }
    }
}

impl Matcher for Symbol {
    fn try_match(
        &self,
        tokenizer: &mut Tokenizer
    ) -> Option<Token> {
        for symbol in self.symbols.clone() {
            let dat = tokenizer.clone().take(symbol.len());

            if dat.size_hint().1.unwrap() != symbol.len() {
                return None;
            }

            if dat.collect::<String>() == symbol {
                tokenizer.advance(symbol.len());

                return Some(
                    Token::new(
                        TokenType::Symbol,
                        TokenPosition::new(0, *tokenizer.index()),
                        symbol,
                    )
                );
            }
        }

        None
    }
}