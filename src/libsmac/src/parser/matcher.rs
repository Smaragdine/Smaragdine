use parser::nodizer::Nodizer;
use parser::node::{Node, NodeType};

use parser::token::TokenType;

pub trait Matcher {
    fn try_match(&self, nodizer: &mut Nodizer) -> Option<Node>;
}

pub struct LiteralMatcher {}

impl Matcher for LiteralMatcher {
    fn try_match(&self, nodizer: &mut Nodizer) -> Option<Node> {
        let token = nodizer.peek().unwrap().clone();
        match nodizer.peek().unwrap().token_type() {
            &TokenType::StringLiteral => {
                nodizer.next();
                let value = token.content().to_owned();
                Some(Node::new(NodeType::StringLiteral(value)))
            },
            &TokenType::CharLiteral => {
                nodizer.next();
                let value = token.content().chars().nth(0).unwrap();
                Some(Node::new(NodeType::CharLiteral(value)))
            },
            &TokenType::IntLiteral => {
                nodizer.next();
                let signed = token.content().starts_with("-");
                let value = match signed {
                    true => token.content().parse::<i64>().unwrap() as u64,
                    false => token.content().parse::<u64>().unwrap(),
                };
                Some(Node::new(NodeType::IntLiteral(value, signed)))
            },
            &TokenType::FloatLiteral => {
                nodizer.next();
                let value = token.content().parse::<f64>().unwrap();
                Some(Node::new(NodeType::FloatLiteral(value)))
            },
            &TokenType::BooleanLiteral => {
                nodizer.next();
                let value = token.content().contains("true");
                Some(Node::new(NodeType::BooleanLiteral(value)))
            },
            _ => None,
        }
    }
}