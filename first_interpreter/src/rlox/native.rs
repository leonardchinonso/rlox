use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    common::Error,
    rlox::{RloxCallable, Value},
};

#[derive(Debug, Clone)]
pub enum NativeCallable {
    ClockFunction(ClockFunction),
}

impl RloxCallable for NativeCallable {
    fn arity(&self) -> usize {
        match self {
            NativeCallable::ClockFunction(c) => c.arity(),
        }
    }

    fn call(
        &self,
        interpreter: &mut super::interpreter::Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        match self {
            NativeCallable::ClockFunction(c) => c.call(interpreter, arguments),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClockFunction;

impl ClockFunction {
    pub fn new() -> ClockFunction {
        ClockFunction
    }
}

impl Display for ClockFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl RloxCallable for ClockFunction {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut super::interpreter::Interpreter,
        _arguments: Vec<super::Value>,
    ) -> Result<Value, Error> {
        let now = SystemTime::now();
        Ok(Value::new(
            now.duration_since(UNIX_EPOCH)
                .expect("Improbable to fail")
                .as_millis(),
        ))
    }
}
