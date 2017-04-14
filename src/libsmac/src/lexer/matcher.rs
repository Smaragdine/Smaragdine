use lexer::Tokenizer;
use lexer::token::{Token, TokenType, TokenPosition};

pub trait Matcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token>;
}

pub struct Whitespace {}

impl Matcher for Whitespace {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut found = false;
        while !tokenizer.end() && tokenizer.peek().unwrap().is_whitespace() {
            found = true;
            tokenizer.next();
        }
        if found {
            Some(Token::new(TokenType::Whitespace,
                            TokenPosition::new(0, 0),
                            String::new()))
        } else {
            None
        }
    }
}

pub struct IntLiteral {}

impl Matcher for IntLiteral {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut integer = "".to_owned();
        let mut start: usize = 0;
        while !tokenizer.end() && tokenizer.peek().unwrap().is_digit(10) {
            if integer.is_empty() {
                start = *tokenizer.index();
            }
            integer.push(tokenizer.next().unwrap())
        }
        if !integer.is_empty() {
            Some(Token::new(TokenType::IntLiteral,
                            TokenPosition::new(start, *tokenizer.index()),
                            integer))
        } else {
            None
        }
    }
}

pub struct Symbol {
    symbols: Vec<String>,
}

impl Symbol {
    pub fn new(symbols: Vec<String>) -> Symbol {
        Symbol { symbols: symbols }
    }
}

impl Matcher for Symbol {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        for symbol in self.symbols.clone() {
            let dat = tokenizer.clone().take(symbol.len());
            if dat.size_hint().1.unwrap() != symbol.len() {
                return None;
            }
            if dat.collect::<String>() == symbol {
                tokenizer.advance(symbol.len());
                return Some(Token::new(TokenType::Symbol,
                                       TokenPosition::new(0, *tokenizer.index()),
                                       symbol));
            }
        }
        None
    }
}

pub struct Identifier {}

impl Matcher for Identifier {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut identifier = String::new();
        let curr = tokenizer.next().unwrap();
        if curr == '_' || curr.is_alphabetic() {
            identifier.push(curr)
        } else {
            return None;
        }
        while !tokenizer.end() {
            let current = *tokenizer.peek().unwrap();
            if !current.is_whitespace() && ("_?!".contains(current) || current.is_alphanumeric()) {
                identifier.push(tokenizer.next().unwrap());
            } else {
                break;
            }
        }
        if !identifier.is_empty() {
            Some(Token::new(TokenType::Identifier,
                            TokenPosition::new(0, *tokenizer.index()),
                            identifier))
        } else {
            None
        }
    }
}