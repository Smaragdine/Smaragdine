use lexer::Tokenizer;
use lexer::token::{Token, TokenType};

/// Matcher.
pub trait Matcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token>;
}

/// A matcher that only matches white-space.
pub struct WhitespaceMatcher {}

impl Matcher for WhitespaceMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut found = false;
        while !tokenizer.end() && tokenizer.peek().unwrap().is_whitespace() {
            found = true;
            tokenizer.next();
        }
        if found {
            Some(Token::new(TokenType::Whitespace,
                            tokenizer.last_position(),
                            String::new()))
        } else {
            None
        }
    }
}

/// A matcher that matches base-10 integer literals.
pub struct IntLiteralMatcher {}

impl Matcher for IntLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut integer = String::new();
        while !tokenizer.end() && tokenizer.peek().unwrap().is_digit(10) {
            integer.push(tokenizer.next().unwrap())
        }
        if !integer.is_empty() {
            Some(Token::new(TokenType::IntLiteral,
                            tokenizer.last_position(),
                            integer))
        } else {
            None
        }
    }
}

/// A matcher that matches constant elements
/// of the specified token type.
pub struct ConstantMatcher {
    token_type: TokenType,
    constants: Vec<String>,
}

impl ConstantMatcher {
    pub fn new(token_type: TokenType, constants: Vec<String>) -> Self {
        ConstantMatcher {
            token_type: token_type,
            constants: constants,
        }
    }
}

impl Matcher for ConstantMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        for constant in self.constants.clone() {
            let dat = tokenizer.clone().take(constant.len());
            if dat.size_hint().1.unwrap() != constant.len() {
                return None;
            }
            if dat.collect::<String>() == constant {
                tokenizer.advance(constant.len());
                return Some(Token::new(self.token_type.clone(),
                                       tokenizer.last_position(),
                                       constant));
            }
        }
        None
    }
}

/// A matcher that matches identifiers.
pub struct IdentifierMatcher {}

impl Matcher for IdentifierMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut identifier = String::new();
        let curr = tokenizer.next().unwrap();
        if curr.is_alphabetic() || curr == '_' {
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
                            tokenizer.last_position(),
                            identifier))
        } else {
            None
        }
    }
}