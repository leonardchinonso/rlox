use crate::rlox::token::Token;

use super::expr::Expr;

/// Represents an assign expression
#[derive(Debug, Clone)]
pub struct Assign {
    name: Token,
    value: Box<Expr>,
}

impl Assign {
    /// Constructs a new Assign expression
    pub fn new(name: Token, value: Expr) -> Assign {
        Assign {
            name,
            value: Box::new(value),
        }
    }

    /// Returns the name of the Assign
    pub fn name(&self) -> Token {
        self.name.clone()
    }

    /// Returns the value of the Assign
    pub fn value(&self) -> Expr {
        *self.value.clone()
    }
}
