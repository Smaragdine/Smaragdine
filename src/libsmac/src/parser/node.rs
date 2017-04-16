#[derive(Debug, Clone)]
pub enum NodeType {
    StringLiteral(String),
    CharLiteral(char),
    IntLiteral(i32),
    FloatLiteral(f32),
    BooleanLiteral(bool),
}

#[derive(Debug, Clone)]
pub struct Node {
    node_type: NodeType,
}

impl Node {
    pub fn new(node_type: NodeType) -> Node {
        Node {
            node_type: node_type,
        }
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }
}