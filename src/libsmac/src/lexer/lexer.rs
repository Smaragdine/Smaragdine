use lexer::Tokenizer;
use lexer::matcher::*;
use lexer::token::{Token, TokenType, TokenPosition};
use lexer::block_tree::{Branch, Chunk, ChunkValue};

use std::str::Chars;

pub fn grab_smaragdine_lexer(data: &mut Chars) -> Lexer {
    let tokenizer = Tokenizer::new(data);
    let mut lexer = Lexer::new(tokenizer);

    let symbols = vec![
        "(",
        ")",
        "[",
        "]",
        "->",
    ].iter().map(|&x| x.to_string()).collect();

    let operators = vec![
        "+"   // add
        ,"-"  // sub
        ,"*"  // mul
        ,"/"  // div
        ,"==" // equality
        ,"="  // assignment
        ,"<<" // shift left
        ,"<=" // less than or equal
        ,"<"  // less than
        ,">>" // shift right
        ,">=" // greater than or equal
        ,">"  // greater than
        ,"~"  // bitwise negate
        ,"!"  // logical not
        ,"&&" // logical and
        ,"&"  // bitwise and
        ,"^"  // bitwise xor
        ,"||" // logical or
        ,"|"  // bitwise or
        ,"%"  // modulo
        ,":"  // type hint
    ].iter().map(|&x| x.to_string()).collect();

    let keywords = vec![
        "let",
        "if",
        "else",
    ].iter().map(|&x| x.to_string()).collect();

    let boolean = vec![
        "true",
        "false",
    ].iter().map(|&x| x.to_string()).collect();

    let matcher_symbol         = ConstantMatcher::new(TokenType::Symbol, symbols);
    let matcher_operator       = ConstantMatcher::new(TokenType::Operator, operators);
    let matcher_keyword        = ConstantMatcher::new(TokenType::Keyword, keywords);
    let matcher_boolean        = ConstantMatcher::new(TokenType::BooleanLiteral, boolean);
    let matcher_whitespace     = WhitespaceMatcher {};
    let matcher_int_literal    = IntLiteralMatcher {};
    let matcher_float_literal  = FloatLiteralMatcher {};
    let matcher_identifier     = IdentifierMatcher {};
    let matcher_string_literal = StringLiteralMatcher {};

    lexer.matchers_mut().push(Box::new(matcher_whitespace));
    lexer.matchers_mut().push(Box::new(matcher_float_literal));
    lexer.matchers_mut().push(Box::new(matcher_int_literal));
    lexer.matchers_mut().push(Box::new(matcher_string_literal));
    lexer.matchers_mut().push(Box::new(matcher_boolean));
    lexer.matchers_mut().push(Box::new(matcher_keyword));
    lexer.matchers_mut().push(Box::new(matcher_identifier));
    lexer.matchers_mut().push(Box::new(matcher_operator));
    lexer.matchers_mut().push(Box::new(matcher_symbol));

    lexer
}

pub fn lex_branch(branch: &Branch) -> Branch {
    let mut lexed_branch = Branch::new(Vec::new());

    for c in branch.value.iter() {
        match c.value() {
            &ChunkValue::Source(ref s) => {
                let chunk = ChunkValue::Tokens(grab_smaragdine_lexer(&mut s.clone().chars()).collect());
                lexed_branch.value.push(Chunk::new(chunk))
            },

            &ChunkValue::Block(ref b) => {
                let chunk = ChunkValue::Block(lex_branch(&b));
                lexed_branch.value.push(Chunk::new(chunk))
            },

            _ => (),
        }
    }

    lexed_branch
}

pub fn flatten_branch(branch: &Branch) -> Vec<Token> {
    let mut flat = Vec::new();

    for c in branch.value.iter() {
        match c.value() {
            &ChunkValue::Tokens(ref t) => flat.append(&mut t.clone()),
            &ChunkValue::Block(ref b)  => flat.push(Token::new(TokenType::Block(flatten_branch(b)), TokenPosition::new(0, 0), "".to_string())),
            _ => continue,
        }
    }

    flat
}

pub struct Lexer {
    tokenizer: Tokenizer,
    matchers: Vec<Box<Matcher>>,
}

impl Lexer {
    pub fn new(tokenizer: Tokenizer) -> Lexer {
        Lexer {
            tokenizer: tokenizer,
            matchers: Vec::new(),
        }
    }

    pub fn match_token(&mut self) -> Option<Token> {
        for matcher in &mut self.matchers {
            match self.tokenizer.try_match_token(matcher.as_ref()) {
                Some(t) => return Some(t),
                None => continue,
            }
        }
        None
    }
    
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
                match self.next() {
                    Some(t) => Some(t),
                    None => None,
                }
            }
            _ => Some(token),
        }
    }
}