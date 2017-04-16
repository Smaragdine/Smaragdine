use lexer::Token;

#[derive(Debug)]
pub enum ChunkValue {
    Source(String),
    Tokens(Vec<Token>),
    Block(Branch),
}

#[derive(Debug)]
pub struct Chunk {
    value: ChunkValue,
}

impl Chunk {
    pub fn new(value: ChunkValue) -> Chunk {
        Chunk {
            value: value,
        }
    }

    pub fn value(&self) -> &ChunkValue {
        &self.value
    }
}

#[derive(Debug)]
pub struct Branch {
    pub value: Vec<Chunk>,
}

impl Branch {
    pub fn new(value: Vec<Chunk>) -> Branch {
        Branch {
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct BlockTree<'a> {
    source: &'a str,
    current_line: usize,
}

impl<'a> BlockTree<'a> {
    pub fn new(source: &str, current_line: usize) -> BlockTree {
        BlockTree {
            source:       source,
            current_line: current_line,
        }
    }

    pub fn collect_indents(&self) -> Vec<(usize, &'a str)> {
        let mut indents = Vec::new();
        let mut lines   = self.source.lines();

        while let Some(line) = lines.next() {
            if line.trim().len() > 0 {
                indents.push((self.indent(&line), line.trim()))
            } 
        }

        indents
    }

    pub fn indent(&self, line: &str) -> usize {
        let mut pos: usize = 0;

        for c in line.chars() {
            if c == ' ' {
                pos += 1
            } else {
                break
            }
        }

        pos
    }

    pub fn tree(&mut self, indents: &Vec<(usize, &'a str)>) -> Branch {
        let mut branch = Branch::new(Vec::new());
        let line       = indents.get(self.current_line);

        let &(base_indent, _) = match line {
            Some(i) => i,
            None    => return branch,
        };

        while self.current_line < indents.len() {
            let (indent, line) = indents[self.current_line];

            if indent == base_indent {

                branch.value.push(Chunk::new(ChunkValue::Source(line.to_owned())))
            
            } else if indent < base_indent {
            
                self.current_line -= 1;

                return branch

            } else if indent > base_indent {
                branch.value.push(Chunk::new(ChunkValue::Block(self.tree(&indents))))
            }

            self.current_line += 1
        }

        branch
    }
}