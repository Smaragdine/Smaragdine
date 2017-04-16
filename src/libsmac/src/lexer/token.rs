use std::fmt;

/// Represents the type of a token
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    IntLiteral,
    FloatLiteral,
    Keyword,
    Symbol,
    Operator,
    Identifier,
    Whitespace,
    StringLiteral,
    CharLiteral,
    LiteralStringLiteral,
    EOF,
}

/// Position of a token
#[derive(Debug, Copy, Clone)]
pub struct TokenPosition {
    pub line: usize,
    pub col: usize,
}

impl Default for TokenPosition {
    fn default() -> Self {
        TokenPosition {
            line: 1,
            col: 0,
        }
    }
}

impl TokenPosition {
    pub fn new(line: usize, col: usize) -> TokenPosition {
        TokenPosition {
            line: line,
            col: col,
        }
    }
}

impl fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ln {}, col {}]", self.line, self.col)
    }
}

/// Token representation
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    position: TokenPosition,
    content: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Token({}, {:?} '{}')",
               self.position,
               self.token_type,
               self.content)
    }
}

impl Token {
    pub fn new(token_type: TokenType, position: TokenPosition, content: String) -> Token {
        Token {
            token_type: token_type,
            position: position,
            content: content,
        }
    }

    // Immutable access
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn position(&self) -> &TokenPosition {
        &self.position
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    // Mutable access
    pub fn token_type_mut(&mut self) -> &mut TokenType {
        &mut self.token_type
    }

    pub fn position_mut(&mut self) -> &mut TokenPosition {
        &mut self.position
    }
}

impl<'a> PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        &self.token_type == other.token_type()
    }

    fn ne(&self, other: &Token) -> bool {
        &self.token_type != other.token_type()
    }
}