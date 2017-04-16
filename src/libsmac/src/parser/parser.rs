use parser::nodizer::Nodizer;
use parser::matcher::*;
use parser::node::Node;

use parser::token::Token;

pub fn grab_smaragdine_parser(data: Vec<Token>) -> Parser {
    let nodizer = Nodizer::new(data);
    let mut parser = Parser::new(nodizer);

    let matcher_literal = LiteralMatcher {};

    parser.matchers_mut().push(Box::new(matcher_literal));
    parser
}

pub struct Parser {
    nodizer: Nodizer,
    matchers: Vec<Box<Matcher>>,
}

impl Parser {
    pub fn new(nodizer: Nodizer) -> Parser {
        Parser {
            nodizer:  nodizer,
            matchers: Vec::new(),
        }
    }

    pub fn match_node(&mut self) -> Option<Node> {
        for matcher in &mut self.matchers {
            match self.nodizer.try_match_node(matcher.as_ref()) {
                Some(t) => return Some(t),
                None    => continue,
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

impl Iterator for Parser {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        if self.nodizer.end() {
            None
        } else {
            Some(self.match_node().unwrap())
        }
    }
}