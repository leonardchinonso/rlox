use crate::{
    errors::Error,
    token::{Token, TokenType},
};

/// This represents a structure for scanning the source file
/// and transforming it into tokens
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect::<Vec<char>>(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Returns the tokens scanned by the scanner.
    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    /// Scans a source file and drafts tokens from it
    pub fn scan_tokens(&mut self) -> Result<(), Error> {
        let mut num_errors = 0usize;
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            if let Err(_) = self.scan_token() {
                num_errors += 1;
            }
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", None, self.line));

        if num_errors == 0 {
            Ok(())
        } else {
            Err(Error::report_syntax(
                None,
                &format!("Found {} compilation errors", num_errors),
            ))
        }
    }

    pub fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            Some('(') => self.add_token(TokenType::LeftParen, None),
            Some(')') => self.add_token(TokenType::RightParen, None),
            Some('{') => self.add_token(TokenType::LeftBrace, None),
            Some('}') => self.add_token(TokenType::RightBrace, None),
            Some(',') => self.add_token(TokenType::Comma, None),
            Some('.') => self.add_token(TokenType::Dot, None),
            Some('-') => self.add_token(TokenType::Minus, None),
            Some('+') => self.add_token(TokenType::Plus, None),
            Some(';') => self.add_token(TokenType::Semicolon, None),
            Some('*') => self.add_token(TokenType::Star, None),
            Some('!') => {
                match self.conditionally_advance('=') {
                    true => self.add_token(TokenType::BangEqual, None),
                    false => self.add_token(TokenType::Bang, None),
                };
            }
            Some('=') => {
                match self.conditionally_advance('=') {
                    true => self.add_token(TokenType::EqualEqual, None),
                    false => self.add_token(TokenType::Equal, None),
                };
            }
            Some('<') => {
                match self.conditionally_advance('=') {
                    true => self.add_token(TokenType::LessEqual, None),
                    false => self.add_token(TokenType::Less, None),
                };
            }
            Some('>') => {
                match self.conditionally_advance('=') {
                    true => self.add_token(TokenType::GreaterEqual, None),
                    false => self.add_token(TokenType::Greater, None),
                };
            }
            Some('/') => {
                match self.conditionally_advance('/') {
                    true => {
                        // a comment goes until the end of the line
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    false => self.add_token(TokenType::Slash, None),
                }
            }
            Some('\n') => self.line += 1,
            // skip over and ignore other whitespaces
            Some(' ') | Some('\r') | Some('\t') => {}
            Some('"') => self.parse_string()?,
            _ => {
                return Err(Error::report_syntax(
                    Some(self.line),
                    "Unexpected character.",
                ))
            }
        };
        Ok(())
    }

    /// Returns true if the current pointer is at EOF
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Looks ahead and returns the next character
    /// as long as EOF is not reached.
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    /// Consumes a character at the current position.
    ///
    /// Returns the character.
    pub fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            eprintln!("Cannot advance as scanner is at EOF. This should not happen");
            return None;
        }

        let c = self.source[self.current];
        self.current += 1;
        Some(c)
    }

    /// Consumes a character at the current position only if the character
    /// is the expected character passed as argument.
    ///
    /// A conditional version of [`Self::advance`]
    ///
    /// Returns a boolean
    pub fn conditionally_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        // consume the current character
        self.current += 1;
        true
    }

    /// Begins to parse a string
    pub fn parse_string(&mut self) -> Result<(), Error> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Error::report_syntax(Some(self.line), "Unterminated string"));
        }

        // consume the closing quotation character (")
        self.advance();

        // trim the surrounding quotes.
        let s = self.source[self.start + 1..self.current + 1]
            .iter()
            .collect::<String>();
        Ok(self.add_token(TokenType::String, Some(&s)))
    }

    /// Creates a new token from the type and literal and pushes it to the
    /// scanner's token list.
    pub fn add_token(&mut self, token_type: TokenType, literal: Option<&str>) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(token_type, &text, literal, self.line));
    }
}
