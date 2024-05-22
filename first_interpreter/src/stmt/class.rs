use crate::common::Error;
use crate::rlox::interpreter::Interpreter;
use crate::rlox::{RloxCallable, Value};
use crate::stmt::Function;
use crate::{expressions::Variable, rlox::Token};

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

impl RloxCallable for Class {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Error> {
        println!("Class called");
        Ok(Value::new(0))
    }
}
