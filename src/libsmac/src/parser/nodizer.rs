use parser::matcher::Matcher;
use parser::node::Node;
use parser::token::Token;

pub struct Snapshot {
    index: usize,
}

impl Snapshot {
    pub fn new(index: usize) -> Snapshot {
        Snapshot {
            index: index,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

pub struct Nodizer {
    index:     usize,
    items:     Vec<Token>,
    snapshots: Vec<Snapshot>
}

impl Iterator for Nodizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.read().cloned()
    }
}

impl Nodizer {
    pub fn new(items: Vec<Token>) -> Nodizer {
        Nodizer {
            index:     0,
            items:     items,
            snapshots: Vec::new(),
        }
    }

    pub fn end(&self) -> bool {
        self.end_n(0)
    }

    pub fn end_n(&self, lookahead: usize) -> bool {
        self.index + lookahead >= self.items.len()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.peek_n(0)
    }

    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        if self.end_n(n) {
            return None
        }
        Some(&self.items[self.index + n])
    }

    pub fn read(&mut self) -> Option<&Token> {
        if self.end() {
            return None;
        }
        self.advance(1);
        Some(&self.items[self.index - 1])
    }

    pub fn advance(&mut self, a: usize) {
        self.index += a;
    }

    pub fn take_snapshot(&mut self) {
        self.snapshots.push(Snapshot::new(self.index));
    }

    pub fn peek_snapshot(&self) -> Option<&Snapshot> {
        self.snapshots.last()
    }

    pub fn rollback_snapshot(&mut self) {
        let snapshot = self.snapshots.pop().unwrap();
        self.index   = snapshot.index();
    }

    pub fn commit_snapshot(&mut self) {
        self.snapshots.pop();
    }

    pub fn try_match_node(&mut self, matcher: &Matcher) -> Option<Node> {
        if self.end() {
            return None
        }
        self.take_snapshot();
        match matcher.try_match(self) {
            Some(t) => {
                self.commit_snapshot();
                Some(t)
            },

            None => {
                self.rollback_snapshot();
                None
            }
        }
    }

    pub fn index(&self) -> &usize {
        &self.index
    }
}