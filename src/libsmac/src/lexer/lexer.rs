use lexer::Tokenizer;
use lexer::matcher::Matcher;
use lexer::token::{Token, TokenType};

pub struct Lexer {
    tokenizer: Tokenizer,
    matchers:  Vec<Box<Matcher>>,
}

impl Lexer {
    pub fn new(
        tokenizer: Tokenizer,
        matchers:  Vec<Box<Matcher>>,
    ) -> Lexer {
        Lexer {
            tokenizer: tokenizer,
            matchers:  matchers,
        }
    }

    pub fn match_token(&mut self) -> Option<Token> {
        for matcher in &mut self.matchers {
            match self.tokenizer.try_match_token(matcher.as_ref()) {
                Some(t) => return Some(t),
                None    => continue,
            }
        }

        None
    }

    // Mutable access
    pub fn matchers(&self) -> &Vec<Box<Matcher>> {
        &self.matchers
    }

    pub fn matchers_mut(&mut self) -> &mut Vec<Box<Matcher>> {
        &mut self.matchers
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = self.match_token().unwrap();

        match *token.token_type() {
            TokenType::EOF => None,
            TokenType::Whitespace => {
                Some(self.next().unwrap())
            },

            _ => Some(token)
        }
    }
}