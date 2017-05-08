use lexer::Tokenizer;
use lexer::token::{Token, TokenType};

macro_rules! token {
    ($tokenizer:expr, $token_type:ident, $accum:expr) => {{
        token!($tokenizer , TokenType::$token_type, $accum)
    }};
    ($tokenizer:expr, $token_type:expr, $accum:expr) => {{
        let tokenizer = $tokenizer as &$crate::lexer::Tokenizer;
        let token_type = $token_type as $crate::lexer::token::TokenType;
        Some(Token::new(token_type, tokenizer.last_position(), $accum))
    }};
}

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
            token!(tokenizer, Whitespace, String::new())
        } else {
            None
        }
    }
}

/// A matcher that matches base-10 integer literals.
pub struct IntLiteralMatcher {}

impl Matcher for IntLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut accum = String::new();
        let negative = tokenizer.peek() == Some(&'-');
        if negative { tokenizer.advance(1) };
        let base = match tokenizer.peek().unwrap() {
            &'0' => {
                match tokenizer.peek_n(1) {
                    Some(chr) => {
                        match chr {
                            &'x' => 16, // base 16 (hexadecimal)
                            &'b' => 2, // base 2 (binary)
                            _ => 10, // base 10 (decimal)
                        }
                    }
                    _ => 10, // base 10 (decimal)
                }
            }
            _ => 10, // base 10 (decimal)
        };
        if base != 10 {
            tokenizer.advance(2); // skip prefix
        }
        while !tokenizer.end() && tokenizer.peek().unwrap().is_digit(base) {
            accum.push(tokenizer.next().unwrap());
        }
        if !accum.is_empty() {
            // Produce token as base-10 string
            let literal: String = if negative {
                match i64::from_str_radix(accum.as_str(), base) {
                    Ok(result) => format!("-{}", result),
                    Err(error) => panic!("Unable to parse integer literal: {}", error)
                }
            } else {
                match u64::from_str_radix(accum.as_str(), base) {
                    Ok(result) => result.to_string(),
                    Err(error) => panic!("Unable to parse integer literal: {}", error)
                }
            };
            token!(tokenizer, IntLiteral, literal)
        } else {
            None
        }
    }
}

/// A matcher that matches float literals.
pub struct FloatLiteralMatcher {}

impl Matcher for FloatLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut accum = String::new();
        let curr = tokenizer.next().unwrap();
        if curr.is_digit(10) {
            accum.push(curr)
        } else if curr == '.' {
            accum.push_str("0.")
        } else {
            return None;
        }
        while !tokenizer.end() {
            let current = *tokenizer.peek().unwrap();
            if !current.is_whitespace() && current.is_digit(10) || current == '.' {
                if current == '.' && accum.contains('.') {
                    panic!("Unexpected decimal point")
                }

                accum.push(tokenizer.next().unwrap());
            } else {
                break
            }
        }
        if accum.chars().last() == Some('.') {
            accum.push('0');
        }
        if accum.contains('.') {
            token!(tokenizer, FloatLiteral, accum)
        } else {
            None
        }
    }
}

/// A matcher that matches string literals.
pub struct StringLiteralMatcher {}

impl Matcher for StringLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut raw_marker = false;
        let delimeter  = match tokenizer.peek().unwrap() {
            &'"'  => Some('"'),
            &'\'' => Some('\''),
            &'r' if tokenizer.peek_n(1) == Some(&'"') => {
                raw_marker = true;
                tokenizer.advance(1); // Skips prefix

                Some('"')
            },
            _ => return None,
        };
        tokenizer.advance(1); // Skips the opening delimiter
        let mut string       = String::new();
        let mut found_escape = false;
        loop {
            if tokenizer.end() {
                break
            }
            if raw_marker {
                if tokenizer.peek().unwrap() == &'"' {
                    break
                }
                string.push(tokenizer.next().unwrap())
            } else {
                if found_escape {
                    string.push(
                        match tokenizer.next().unwrap() {
                            c @ '\\' | c @ '\'' | c @ '"' => c,
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            s => panic!("Invalid character escape: {}", s),
                        }
                    );
                    found_escape = false
                } else {
                    match tokenizer.peek().unwrap() {
                        &'\\' => {
                            tokenizer.next();
                            found_escape = true
                        },
                        &c if &c == &delimeter.unwrap() => break,
                        _ => string.push(tokenizer.next().unwrap()),
                    }
                }
            }
        }
        tokenizer.advance(1); // Skips the closing delimeter
        match delimeter.unwrap() {
            '"'  => {
                token!(tokenizer, StringLiteral, string)
            },
            _ => {
                if string.len() == 1 {
                    token!(tokenizer, CharLiteral, string)
                } else {
                    panic!("Invalid char literal")
                }
            },
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
                return token!(tokenizer, self.token_type.clone(), constant)
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
            token!(tokenizer, Identifier, identifier)
        } else {
            None
        }
    }
}
