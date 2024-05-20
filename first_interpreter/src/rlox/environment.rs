use std::collections::HashMap;

use crate::{
    common::Error,
    rlox::{Token, TokenLiteral},
};

#[derive(Debug, Clone)]
/// Represents some kind of storage for variables to values
pub(crate) struct Environment {
    state: HashMap<String, TokenLiteral>,
    parent: Option<Box<Environment>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            state: Default::default(),
            parent: Default::default(),
        }
    }
}

impl Environment {
    /// Constructs a new Environment
    pub(crate) fn new() -> Environment {
        Environment {
            state: HashMap::new(),
            parent: None,
        }
    }

    /// Constructs a new Environment with the passed in Environment
    /// as its enclosing environment (parent)
    pub(crate) fn with_parent(parent: Environment) -> Environment {
        Environment {
            state: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Defines a new variable by storing it in the Environment table
    pub(crate) fn define(&mut self, name: String, value: TokenLiteral) {
        self.state.insert(name, value);
    }

    /// Returns the value of a variable
    pub(crate) fn get(&self, name: &Token) -> Result<TokenLiteral, Error> {
        // check this environment for the token
        let lexeme = name.lexeme();
        if let Some(v) = self.state.get(&lexeme) {
            return Ok(v.clone());
        }

        // check the ancestor environment
        if let Some(v) = self.parent.clone() {
            return v.get(name);
        }

        Err(Error::report_runtime(
            name.clone(),
            &format!("Undefined variable '{}'.", lexeme),
        ))
    }

    /// Assigns a new value to a variable
    /// Errors if the variable has not been declared before
    pub(crate) fn assign(&mut self, name: Token, value: TokenLiteral) -> Result<(), Error> {
        let lexeme = name.lexeme();
        if self.state.contains_key(&lexeme) {
            self.state.insert(name.lexeme(), value);
            return Ok(());
        }

        // check the ancestor environment
        if let Some(mut v) = self.parent.clone() {
            return v.assign(name, value);
        }

        Err(Error::report_runtime(
            name,
            &format!("Undefined variable '{}'.", lexeme),
        ))
    }
}
