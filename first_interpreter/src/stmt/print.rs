use crate::expressions::Expr;

/// Represents a Print statement
#[derive(Debug, Clone)]
pub struct Print {
    expression: Expr,
}

impl Print {
    /// Construct a new Print statement
    pub fn new(expression: Expr) -> Print {
        Print { expression }
    }

    /// Return the expression
    pub fn expression(&self) -> Expr {
        self.expression.clone()
    }
}
