use crate::{expressions::Expr, stmt::Stmt};

/// Represents an if statement
#[derive(Debug, Clone)]
pub struct If {
    condition: Expr,
    then_branch: Box<Stmt>,
    // not all if constructs have an else branch
    else_branch: Option<Box<Stmt>>,
}

impl If {
    /// Construct a new If statement
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> If {
        If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(|b| Box::new(b)),
        }
    }

    /// Return the condition
    pub fn condition(&self) -> Expr {
        self.condition.clone()
    }

    /// Return the then branch
    pub fn then_branch(&self) -> Stmt {
        *self.then_branch.clone()
    }

    /// Return the else branch
    pub fn else_branch(&self) -> Option<Stmt> {
        self.else_branch.clone().map(|b| *b)
    }
}
