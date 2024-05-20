use crate::rlox::token::Token;

use super::expr::Expr;

/// Represents a logical expression
#[derive(Debug, Clone)]
pub struct Logical {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Logical {
    /// Constructs a new logical expression
    pub fn new(left: Expr, operator: Token, right: Expr) -> Logical {
        Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    /// Returns the left operand
    pub fn left(&self) -> Expr {
        let left = self.left.clone();
        *left
    }

    /// Returns the operator
    pub fn operator(&self) -> &Token {
        &self.operator
    }

    /// Returns the right operand
    pub fn right(&self) -> Expr {
        let right = self.right.clone();
        *right
    }
}
