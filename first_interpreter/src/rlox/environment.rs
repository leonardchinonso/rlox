use std::collections::HashMap;

use crate::{
    common::Error,
    rlox::{Token, TokenLiteral},
};

/// Represents some kind of storage for variables to values
pub(crate) struct Environment(HashMap<String, TokenLiteral>);

impl Environment {
    /// Constructs a new Environment
    pub(crate) fn new() -> Environment {
        Environment(HashMap::new())
    }

    /// Defines a new variable by storing it in the Environment table
    pub(crate) fn define(&mut self, name: String, value: TokenLiteral) {
        self.0.insert(name, value);
    }

    /// Returns the value of a variable
    pub(crate) fn get(&self, name: Token) -> Result<TokenLiteral, Error> {
        let lexeme = name.lexeme();
        if let Some(v) = self.0.get(&lexeme) {
            return Ok(v.clone());
        }
        Err(Error::report_runtime(
            name,
            &format!("Undefined variable '{}'.", lexeme),
        ))
    }
}
