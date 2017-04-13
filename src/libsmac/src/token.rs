use std::fmt;

/// Represents the type of a token
#[derive(Debug, PartialEq)]
pub enum TokenType {
    White,
    EOF,
}

/// Position of a token
pub struct TokenPosition {
    line: u32,
    col:  u32,
}

impl fmt::Display for TokenPosition {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "[ln {}, col {}]", self.line, self.col)
    }
}

/// Token representation
pub struct Token<'a> {
    token_type: TokenType,
    position:   TokenPosition,
    content:    &'a mut str,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "Token({}, {:?}({}))", self.position, self.token_type, self.content)
    }
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        position:   TokenPosition,
        content:    &mut str,
    ) -> Token {
        Token {
            token_type: token_type,
            position:   position,
            content:    content,
        }
    }

    // Immutable access
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn position(&self) -> &TokenPosition {
        &self.position
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }

    // Mutable access
    pub fn token_type_mut(&mut self) -> &mut TokenType {
        &mut self.token_type
    }

    pub fn position_mut(&mut self) -> &mut TokenPosition {
        &mut self.position
    }
    
    pub fn content_mut(&mut self) -> &mut str {
        &mut self.content
    }
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Token) -> bool {
        &self.token_type == other.token_type()
    }

    fn ne(&self, other: &Token) -> bool {
        &self.token_type != other.token_type()
    }
}