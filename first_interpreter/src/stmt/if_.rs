use crate::{expressions::Expr, stmt::Stmt};

/// Represents an if statement
#[derive(Debug, Clone)]
pub struct If {
    condition: Expr,
    then_branch: Vec<Stmt>,
    else_branch: Vec<Stmt>,
}

impl If {
    /// Construct a new If statement
    pub fn new(condition: Expr, then_branch: Vec<Stmt>, else_branch: Vec<Stmt>) -> If {
        If {
            condition,
            then_branch,
            else_branch,
        }
    }

    /// Return the condition
    pub fn condition(&self) -> Expr {
        self.condition.clone()
    }

    /// Return the then branch
    pub fn then_branch(&self) -> Vec<Stmt> {
        self.then_branch.clone()
    }

    /// Return the else branch
    pub fn else_branch(&self) -> Vec<Stmt> {
        self.else_branch.clone()
    }
}
