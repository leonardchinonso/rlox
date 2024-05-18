use crate::{expressions::Expr, stmt::Stmt};

/// Represents a While statement
#[derive(Debug, Clone)]
pub struct While {
    condition: Expr,
    body: Box<Stmt>,
}

impl While {
    /// Construct a new While statment
    pub fn new(condition: Expr, body: Box<Stmt>) -> While {
        While { condition, body }
    }

    /// Returns the condition
    pub fn condition(&self) -> Expr {
        self.condition.clone()
    }

    /// Returns the body
    pub fn body(&self) -> Box<Stmt> {
        self.body.clone()
    }
}
