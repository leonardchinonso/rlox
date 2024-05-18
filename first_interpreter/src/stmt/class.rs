use crate::{expressions::Variable, rlox::Token};

use crate::stmt::Function;

/// Represents a Class
#[derive(Debug, Clone)]
pub struct Class {
    name: Token,
    superclass: Variable,
    methods: Function,
}

impl Class {
    /// Construct a new Class
    pub fn new(name: Token, superclass: Variable, methods: Function) -> Class {
        Class {
            name,
            superclass,
            methods,
        }
    }

    /// Returns the name
    pub fn name(&self) -> Token {
        self.name.clone()
    }

    /// Returns the superclass
    pub fn superclass(&self) -> Variable {
        self.superclass.clone()
    }

    /// Returns the methods
    pub fn methods(&self) -> Function {
        self.methods.clone()
    }
}
