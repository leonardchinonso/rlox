use crate::{
    expressions::{literal::Literal, Expr},
    rlox::{Token, TokenLiteral, Value},
};

/// Represents a Return statement
#[derive(Debug, Clone)]
pub struct Var {
    name: Token,
    initializer: Expr,
    is_initialized: bool,
}

impl Var {
    /// Construct a new variable Var
    pub fn new(name: Token, initializer: Option<Expr>) -> Var {
        let is_initialized = match initializer {
            Some(_) => true,
            None => false,
        };

        let initializer =
            initializer.unwrap_or(Expr::Literal(Literal::new(Value::new(TokenLiteral::Nil))));
        Var {
            name,
            initializer,
            is_initialized,
        }
    }

    /// Returns the name
    pub fn name(&self) -> Token {
        self.name.clone()
    }

    /// Returns the initializer
    pub fn initializer(&self) -> Expr {
        self.initializer.clone()
    }

    /// Returns true if the variable was initialized and false otherwise
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
