use crate::rlox::token::Token;

use super::expr::Expr;

/// Represents a Unary expression
#[derive(Debug, Clone)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Unary {
    /// Constructs a Unary
    pub fn new(operator: Token, right: Expr) -> Unary {
        Unary {
            operator,
            right: Box::new(right),
        }
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
