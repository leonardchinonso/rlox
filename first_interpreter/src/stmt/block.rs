use crate::stmt::Stmt;

/// Represents a block of statements
#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Stmt>,
}

impl Block {
    /// Construct a new Block
    pub fn new(statements: Vec<Stmt>) -> Block {
        Block { statements }
    }

    /// Return the embedded statements
    pub fn statements(&self) -> Vec<Stmt> {
        self.statements.clone()
    }
}
