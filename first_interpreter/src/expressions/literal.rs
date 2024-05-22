use crate::rlox::Value;

/// This represents a literal value used in expressions
#[derive(Debug, Clone)]
pub struct Literal {
    value: Value,
}

impl Literal {
    /// Constructs a new Literal
    pub fn new(value: Value) -> Literal {
        Literal { value }
    }

    /// Returns the literal value
    pub fn value(&self) -> Value {
        self.value.clone()
    }
}
