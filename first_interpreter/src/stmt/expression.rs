use crate::expressions::Expr;

/// Represents an Expression
#[derive(Debug, Clone)]
pub struct Expression {
    expression: Expr,
}

impl Expression {
    /// Construct a new Expression
    pub fn new(expression: Expr) -> Expression {
        Expression { expression }
    }

    /// Return the expression
    pub fn expression(&self) -> Expr {
        self.expression.clone()
    }
}
