use crate::lox::token::TokenLiteral;

/// This represents a literal value used in expressions
#[derive(Debug, Clone)]
pub struct Literal {
    value: TokenLiteral,
}

impl Literal {
    /// Constructs a new Literal
    pub fn new(value: TokenLiteral) -> Literal {
        Literal { value }
    }

    /// Returns the literal value
    pub fn value(&self) -> TokenLiteral {
        self.value.clone()
    }
}
