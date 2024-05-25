// use std::ops::{Deref, DerefMut};

/// Implements a Stack using a vector
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    /// Creates a new stack
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.0.get_mut(self.0.len() - 1)
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
}

// impl<T> Deref for Stack<T> {
//     type Target = Vec<T>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> DerefMut for Stack<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// TODO: write tests for the stack
#[cfg(test)]
mod tests {
    #[test]
    fn test_new_stack() {}
}
