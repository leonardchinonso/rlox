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
pub struct RloxFunction {
    declaration: Function,
    closure: Rc<RefCell<Environment>>,
}

impl RloxFunction {
    pub fn new(declaration: Function, closure: Rc<RefCell<Environment>>) -> RloxFunction {
        RloxFunction {
            declaration,
            closure,
        }
    }
}

impl Display for RloxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme())
    }
}

impl RloxCallable for RloxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Error> {
        let mut environment = Environment::with_parent(self.closure.clone());
        for i in 0..self.declaration.params.len() {
            environment.define(self.declaration.params[i].lexeme(), arguments[i].clone())
        }
        match interpreter.execute_block(self.declaration.body(), Rc::new(RefCell::new(environment)))
        {
            Ok(_) => Ok(Value::new(TokenLiteral::Nil)),
            Err(Error::Return(ret_val)) => Ok(ret_val),
            Err(err) => Err(err),
        }
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
