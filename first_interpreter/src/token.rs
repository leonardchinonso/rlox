/// This represents a chunk of a source file, a token.
#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: &str, literal: &str, line: u32) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            literal: literal.to_string(),
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.kind, self.lexeme, self.literal)
    }
}

/// Represents the different possible token types
#[derive(Debug)]
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
    Number,

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
