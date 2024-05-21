use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    common::Error,
    rlox::{Token, TokenLiteral},
};

#[derive(Debug, Clone)]
/// Represents some kind of storage for variables to values
pub(crate) struct Environment {
    state: HashMap<String, TokenLiteral>,
    parent: Option<Rc<RefCell<Environment>>>,
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
    pub(crate) fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            state: HashMap::new(),
            parent: Some(parent),
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
            return v.borrow().get(name);
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
            self.state.insert(lexeme.clone(), value);
            return Ok(());
        }

        // check the ancestor environment
        let parent = self.parent.take();
        if let Some(parent_environment) = parent {
            parent_environment.borrow_mut().assign(name, value)?;
            self.parent = Some(parent_environment);
            return Ok(());
        }

        Err(Error::report_runtime(
            name,
            &format!("Undefined variable '{}'.", lexeme),
        ))
    }
}
