use crate::token::{Token, TokenType};

/// This represents a structure for scanning the source file
/// and transforming it into tokens
pub struct Scanner;

impl Scanner {
    pub fn new(_source: String) -> Self {
        Self {}
    }

    /// Scans a source file and drafts tokens from it
    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}
