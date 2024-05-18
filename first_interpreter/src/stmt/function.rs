use crate::{rlox::Token, stmt::Stmt};

/// Represents a Function
#[derive(Debug, Clone)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl Function {
    /// Construct a new Function
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Function {
        Function { name, params, body }
    }

    /// Return the name of the function
    pub fn name(&self) -> Token {
        self.name.clone()
    }

    /// Return the parameters of the function
    pub fn params(&self) -> Vec<Token> {
        self.params.clone()
    }

    /// Return the body of the function
    pub fn body(&self) -> Vec<Stmt> {
        self.body.clone()
    }
}
