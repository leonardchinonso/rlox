use crate::{
    common::Error,
    stmt::{Class, Function},
};
use downcast::{downcast, Any};
use dyn_clone::{clone_box, clone_trait_object, DynClone};
use std::{any::TypeId, fmt};

pub trait AnyCloneable: DynClone + Any {}
clone_trait_object!(AnyCloneable);
downcast!(dyn AnyCloneable);

impl<T: Clone + Any> AnyCloneable for T {}

impl fmt::Debug for dyn AnyCloneable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyCloneable").finish_non_exhaustive()
    }
}

#[derive(Debug)]
/// Represents a Value that can be of any type
pub struct Value(Box<dyn AnyCloneable>);

impl Clone for Value {
    fn clone(&self) -> Self {
        Self(clone_box(self.0.as_ref()))
    }
}

impl Value {
    /// Constructs a new Value
    pub fn new<T: Any + Clone>(inner: T) -> Value {
        Value(Box::new(inner))
    }

    /// Returns the concrete type of this Value
    ///
    /// Returns an error if the wrong concrete type is asked for
    pub fn owned<T: Any + Clone>(self) -> Result<T, Error> {
        let boxed = self
            .0
            .downcast()
            .map_err(|_| Error::report_generic("Failed to convert to concrete type"))?;
        Ok(*boxed)
    }

    /// Returns a reference to the concrete type of this value
    ///
    /// Returns None if the wrong concrete type is asked for
    pub fn borrowed<T: Any + Clone>(&self) -> Result<&T, Error> {
        let t = self
            .0
            .downcast_ref()
            .map_err(|_| Error::report_generic("Failed to convert to concrete type"))?;
        Ok(t)
    }

    /// Checks that the current Value is of a type
    pub fn is<T: Any>(&self) -> bool {
        self.0.is::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rlox::{native::ClockFunction, token::TokenType, NativeCallable, Token, TokenLiteral},
        stmt::{Class, Function},
    };

    #[test]
    fn test_new() {
        let value = Value::new(TokenLiteral::Integer(10i32));
        let inner = value.0.downcast::<TokenLiteral>();
        assert!(inner.is_ok());
        let inner = inner.unwrap();
        assert_eq!(inner, Box::new(TokenLiteral::Integer(10i32)));
    }

    #[test]
    fn test_is_type() {
        let value = Value::new(TokenLiteral::Boolean(false));
        assert!(!value.is::<bool>());
        assert!(value.is::<TokenLiteral>());
        assert!(!value.is::<i32>());

        let value = Value::new(Box::new(Function::new(
            Token::new(TokenType::Nil, "", TokenLiteral::Nil, 0),
            vec![],
            vec![],
        )));
        assert!(!value.is::<Function>());
        assert!(!value.is::<Class>());
        assert!(value.is::<Box<Function>>());

        let value = Value::new(NativeCallable::ClockFunction(ClockFunction::new()));
        assert!(!value.is::<Function>());
        assert!(!value.is::<Class>());
        assert!(value.is::<NativeCallable>());
    }

    #[test]
    fn test_owned() {
        let value = Value::new(TokenLiteral::Nil);
        let owned = value.owned::<TokenLiteral>();
        assert!(owned.is_ok());
        let owned = owned.unwrap();
        assert_eq!(owned, TokenLiteral::Nil);
    }

    #[test]
    fn test_borrowed() {
        let value = Value::new(TokenLiteral::Boolean(false));
        let owned = value.borrowed::<TokenLiteral>();
        assert!(owned.is_ok());
        let owned = owned.unwrap();
        assert_eq!(owned, &TokenLiteral::Boolean(false));
    }

    #[test]
    fn test_clone() {
        let value = Value::new(TokenLiteral::Integer(10i32));
        let cloned_value = value.clone();
        let value = Value::new(TokenLiteral::Nil);

        let value = value.owned::<TokenLiteral>().unwrap();
        assert_eq!(value, TokenLiteral::Nil);
        let cloned_value = cloned_value.owned::<TokenLiteral>().unwrap();
        assert_eq!(cloned_value, TokenLiteral::Integer(10i32));
    }
}
