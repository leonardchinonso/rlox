use crate::{expressions::Expr, rlox::Token};

/// Represents a Return statement
#[derive(Debug, Clone)]
pub struct Var {
    name: Token,
    initializer: Option<Expr>,
}

impl Var {
    /// Construct a new variable Var
    pub fn new(name: Token, initializer: Option<Expr>) -> Var {
        Var { name, initializer }
    }

    /// Returns the name
    pub fn name(&self) -> Token {
        self.name.clone()
    }

    /// Returns the initializer
    pub fn initializer(&self) -> Option<Expr> {
        self.initializer.clone()
    }
}
