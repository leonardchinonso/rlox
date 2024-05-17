use crate::lox::token::Token;

use super::expr::Expr;

/// Represents an assign expression
#[derive(Debug, Clone)]
pub struct Assign {
    name: Token,
    value: Box<Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Assign {
        Assign {
            name,
            value: Box::new(value),
        }
    }
}
