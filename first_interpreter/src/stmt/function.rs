use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    common::Error,
    rlox::{
        environment::Environment, interpreter::Interpreter, RloxCallable, Token, TokenLiteral,
        Value,
    },
    stmt::Stmt,
};

/// Represents a wrapper over the Function to keep the
/// interpreter logic separate from the front-end's syntax classes
#[derive(Debug, Clone)]
pub struct RloxFunction(Function);

impl RloxFunction {
    pub fn new(f: Function) -> RloxFunction {
        RloxFunction(f)
    }
}

impl Display for RloxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.0.name.lexeme())
    }
}

impl RloxCallable for RloxFunction {
    fn arity(&self) -> usize {
        self.0.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Error> {
        let mut environment = Environment::with_parent(interpreter.globals());
        for i in 0..self.0.params.len() {
            environment.define(self.0.params[i].lexeme(), arguments[i].clone())
        }
        interpreter.execute_block(self.0.body(), Rc::new(RefCell::new(environment)))?;
        Ok(Value::new(()))
    }
}

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
