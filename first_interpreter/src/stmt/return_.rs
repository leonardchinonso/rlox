use crate::{expressions::Expr, rlox::Token};

/// Represents a Return statement
#[derive(Debug, Clone)]
pub struct Return {
    keyword: Token,
    value: Expr,
}

impl Return {
    /// Construct a new Return statement
    pub fn new(keyword: Token, value: Expr) -> Return {
        Return { keyword, value }
    }

    /// Returns the keyword
    pub fn keyword(&self) -> Token {
        self.keyword.clone()
    }

    /// Returns the value
    pub fn value(&self) -> Expr {
        self.value.clone()
    }
}
