use crate::rlox::token::Token;

/// Represents a Variable expression
#[derive(Debug, Clone)]
pub struct Variable {
    name: Token,
}

impl Variable {
    /// Constructs a Variable
    pub fn new(name: Token) -> Variable {
        Variable { name }
    }

    /// Returns the name of the variable
    pub fn name(&self) -> &Token {
        &self.name
    }
}
