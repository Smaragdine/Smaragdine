use lexer::matcher::Matcher;
use lexer::token::{Token, TokenType, TokenPosition};

#[derive(Clone, Debug)]
struct Snapshot {
    index: usize,
}

impl Snapshot {
    pub fn new(index: usize) -> Snapshot {
        Snapshot { index: index }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Clone, Debug)]
pub struct Tokenizer {
    index: usize,
    items: Vec<char>,
    snapshots: Vec<Snapshot>,
}

impl Iterator for Tokenizer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.read().cloned()
    }
}

impl Tokenizer {
    pub fn new(items: &mut Iterator<Item = char>) -> Tokenizer {
        Tokenizer {
            index: 0,
            items: items.collect(),
            snapshots: Vec::new(),
        }
    }

    pub fn end(&self) -> bool {
        self.index >= self.items.len()
    }

    pub fn peek(&self) -> Option<&char> {
        if self.end() {
            return None;
        }

        Some(&self.items[self.index])
    }

    pub fn read(&mut self) -> Option<&char> {
        if self.end() {
            return None;
        }

        let v = Some(&self.items[self.index]);
        self.index += 1;
        v
    }

    pub fn advance(&mut self, a: usize) {
        self.index += a;
    }

    pub fn take_snapshot(&mut self) {
        self.snapshots.push(Snapshot::new(self.index));
    }

    pub fn rollback_snapshot(&mut self) {
        self.index = self.snapshots.pop().unwrap().index()
    }

    pub fn commit_snapshot(&mut self) {
        self.snapshots.pop();
    }

    pub fn try_match_token(&mut self, matcher: &Matcher) -> Option<Token> {
        if self.end() {
            return Some(Token::new(TokenType::EOF,
                                   TokenPosition::new(self.index, self.index),
                                   String::new()));
        }

        self.take_snapshot();

        match matcher.try_match(self) {
            Some(t) => {
                self.commit_snapshot();
                Some(t)
            }

            None => {
                self.rollback_snapshot();
                None
            }
        }
    }

    // Immutable access
    pub fn index(&self) -> &usize {
        &self.index
    }
}