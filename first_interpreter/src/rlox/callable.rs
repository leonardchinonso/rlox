use crate::{
    common::Error,
    rlox::{Interpreter, Value},
};

/// Represents an interface for callable objects in Rlox
pub trait RloxCallable {
    /// Returns the number of arguments for the callable object
    fn arity(&self) -> usize;
    /// Performs the set of statements grouped under this callable
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Error>;
}

// /// Represents callable types in Rlox
// #[derive(Debug, Clone)]
// pub enum Callable<T: RloxCallable> {
//     Function(Function),
//     Class(Class),
//     Builtin(T),
// }

// impl<T> Callable<T>
// where
//     T: RloxCallable,
// {
//     pub fn from_value(value: Value) -> Callable<T> {
//         if value.is::<Callable<Function>>() {}
//         if value.borrowed::<Callable<Function>>().is_ok() {

//         }

//         Callable::Function(Function::new(
//             Token::new(
//                 super::token::TokenType::And,
//                 "",
//                 super::TokenLiteral::Float(2f64),
//                 0,
//             ),
//             vec![],
//             vec![],
//         ))
//     }
// }

// impl<T> RloxCallable for Callable<T>
// where
//     T: RloxCallable,
// {
//     fn arity(&self) -> usize {
//         match self {
//             Callable::Function(f) => f.arity(),
//             Callable::Class(c) => c.arity(),
//             Callable::Builtin(t) => t.arity(),
//         }
//     }

//     fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Error> {
//         match self {
//             Callable::Function(v) => v.call(interpreter, arguments),
//             Callable::Class(v) => v.call(interpreter, arguments),
//             Callable::Builtin(v) => v.call(interpreter, arguments),
//         }
//     }
// }
