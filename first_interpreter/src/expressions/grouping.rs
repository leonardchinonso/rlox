use super::expr::Expr;

/// Represents a Grouping expression
#[derive(Debug, Clone)]
pub struct Grouping {
    expression: Box<Expr>,
}

impl Grouping {
    /// Constructs a new Grouping
    pub fn new(expression: Expr) -> Grouping {
        Grouping {
            expression: Box::new(expression),
        }
    }

    /// Returns the expression
    pub fn expression(&self) -> Expr {
        let expr = self.expression.clone();
        *expr
    }
}
