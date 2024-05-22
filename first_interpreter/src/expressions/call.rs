use crate::rlox::token::Token;

use super::expr::Expr;

/// Represents a binary expression
#[derive(Debug, Clone)]
pub struct Call {
    callee: Box<Expr>,
    paren: Token,
    arguments: Vec<Expr>,
}

impl Call {
    /// Constructs a new binary expression
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Call {
        Call {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }

    /// Returns the callee
    pub fn callee(&self) -> Expr {
        *self.callee.clone()
    }

    /// Returns the paren
    pub fn paren(&self) -> &Token {
        &self.paren
    }

    /// Returns the arguments
    pub fn arguments(&self) -> &Vec<Expr> {
        &self.arguments
    }
}
