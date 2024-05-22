use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::Debug};

/// This represents a chunk of a source file, a token.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: TokenLiteral,
    line: u32,
}

impl Token {
    /// Constructs a new token
    pub fn new(kind: TokenType, lexeme: &str, literal: TokenLiteral, line: u32) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }

    /// Returns the lexeme
    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }

    /// Returns the kind
    pub fn kind(&self) -> TokenType {
        self.kind.clone()
    }

    /// Returns the literal
    pub fn literal(&self) -> TokenLiteral {
        self.literal.clone()
    }

    /// Returns the line
    pub fn line(&self) -> u32 {
        self.line
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
    }
}

/// Represents the different possible token types
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Integer,
    Float,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub static KEYWORDS: Lazy<HashMap<&str, TokenType>> = Lazy::new(|| {
    HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ])
});

/// Represents the possible literals for a token
/// This literal is the actual value of the token
///
/// Reserved keywords e.g Identifiers do not have a token literal
#[derive(Debug, Clone, PartialEq)]
pub enum TokenLiteral {
    /// Represents a string literal
    String(String),
    /// Represents an integer literal
    Integer(i32),
    /// Represents a float literal
    Float(f64),
    /// Represents a boolean literal
    Boolean(bool),
    /// Represents token without any literals
    Nil,
}

impl TokenLiteral {
    /// Prints the inner literal to console
    pub fn print(&self) {
        match self {
            TokenLiteral::String(v) => println!("{:?}", v),
            TokenLiteral::Integer(v) => println!("{:?}", v),
            TokenLiteral::Float(v) => println!("{:?}", v),
            TokenLiteral::Boolean(v) => println!("{:?}", v),
            TokenLiteral::Nil => println!("nil"),
        }
    }
}

impl std::fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenLiteral::String(v) => write!(f, "{:?}", v),
            TokenLiteral::Integer(v) => write!(f, "{}", v),
            TokenLiteral::Float(v) => write!(f, "{:?}", v),
            TokenLiteral::Boolean(v) => write!(f, "{}", v),
            TokenLiteral::Nil => write!(f, "nil"),
        }
    }
}
