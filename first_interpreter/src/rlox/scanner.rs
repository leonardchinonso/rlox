use crate::{
    common::errors::Error,
    rlox::token::{Token, TokenLiteral, TokenType, KEYWORDS},
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
    ///
    /// Returns the scanned tokens if no error happened
    /// Returns an Error of the ScannerError variant
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut num_errors = 0usize;
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            if let Err(_) = self.scan_token() {
                num_errors += 1;
            }
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", TokenLiteral::Nil, self.line));

        if num_errors == 0 {
            Ok(self.tokens())
        } else {
            Err(Error::report_generic(&format!(
                "Found {} compilation errors",
                num_errors
            )))
        }
    }

    /// Figures out the kind of lexeme currently being looked at
    pub fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            Some('(') => self.add_token(TokenType::LeftParen, TokenLiteral::Nil),
            Some(')') => self.add_token(TokenType::RightParen, TokenLiteral::Nil),
            Some('{') => self.add_token(TokenType::LeftBrace, TokenLiteral::Nil),
            Some('}') => self.add_token(TokenType::RightBrace, TokenLiteral::Nil),
            Some(',') => self.add_token(TokenType::Comma, TokenLiteral::Nil),
            Some('.') => self.add_token(TokenType::Dot, TokenLiteral::Nil),
            Some('-') => self.add_token(TokenType::Minus, TokenLiteral::Nil),
            Some('+') => self.add_token(TokenType::Plus, TokenLiteral::Nil),
            Some(';') => self.add_token(TokenType::Semicolon, TokenLiteral::Nil),
            Some('*') => self.add_token(TokenType::Star, TokenLiteral::Nil),
            Some('!') => {
                match self.conditionally_advance('=') {
                    true => self.add_token(TokenType::BangEqual, TokenLiteral::Nil),
                    false => self.add_token(TokenType::Bang, TokenLiteral::Nil),
                };
            }
            Some('=') => {
                match self.conditionally_advance('=') {
                    true => self.add_token(TokenType::EqualEqual, TokenLiteral::Nil),
                    false => self.add_token(TokenType::Equal, TokenLiteral::Nil),
                };
            }
            Some('<') => {
                match self.conditionally_advance('=') {
                    true => self.add_token(TokenType::LessEqual, TokenLiteral::Nil),
                    false => self.add_token(TokenType::Less, TokenLiteral::Nil),
                };
            }
            Some('>') => {
                match self.conditionally_advance('=') {
                    true => self.add_token(TokenType::GreaterEqual, TokenLiteral::Nil),
                    false => self.add_token(TokenType::Greater, TokenLiteral::Nil),
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
                    false => self.add_token(TokenType::Slash, TokenLiteral::Nil),
                }
            }
            Some('"') => self.parse_string()?,
            Some('\n') => self.line += 1,
            // skip over and ignore other whitespaces
            Some(' ') | Some('\r') | Some('\t') => {}
            Some(c) => {
                if c.is_numeric() {
                    self.parse_number();
                } else if c.is_alphabetic() || c == '_' {
                    self.parse_identifier();
                } else {
                    return Err(Error::report_syntax(
                        self.line,
                        &format!("Unexpected character '{}'", c),
                    ));
                }
            }
            None => {
                return Err(Error::report_syntax(
                    self.line,
                    "Advancing at illegal position.",
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

    /// Looks 2 characters ahead and returns the character
    /// as long as EOF is not reached.
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
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

    /// Parses a string
    pub fn parse_string(&mut self) -> Result<(), Error> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Error::report_syntax(self.line, "Unterminated string"));
        }

        // consume the closing quotation character (")
        self.advance();

        // trim the surrounding quotes.
        let s = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();

        Ok(self.add_token(TokenType::String, TokenLiteral::String(s)))
    }

    /// Parses a numerical value
    pub fn parse_number(&mut self) {
        let mut token_type = TokenType::Integer;

        while self.peek().is_numeric() {
            self.advance();
        }

        // look for a fractional part
        if self.peek() == '.' && self.peek_next().is_numeric() {
            token_type = TokenType::Float;

            // consume the "."
            self.advance();

            // consume the numbers after
            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let digit = &self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        if token_type == TokenType::Integer {
            let parsed_digit: i32 = digit.parse().expect("should be a valid integer");
            self.add_token(token_type, TokenLiteral::Integer(parsed_digit));
        } else {
            let parsed_digit: f64 = digit.parse().expect("should be a valid float");
            self.add_token(token_type, TokenLiteral::Float(parsed_digit));
        }
    }

    /// Parses an identifier or keyword
    pub fn parse_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        let token_type = match KEYWORDS.get(text.as_str()) {
            Some(tt) => tt.clone(),
            None => TokenType::Identifier,
        };

        self.add_token(token_type, TokenLiteral::Nil);
    }

    /// Creates a new token from the type and literal and pushes it to the
    /// scanner's token list.
    pub fn add_token(&mut self, token_type: TokenType, literal: TokenLiteral) {
        let text = match literal {
            // trim to remove quotes if its a string literal
            TokenLiteral::String(_) => &self.source[self.start + 1..self.current - 1],
            _ => &self.source[self.start..self.current],
        };
        let text = text.iter().collect::<String>();
        self.tokens
            .push(Token::new(token_type, &text, literal, self.line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scanner() {
        let scanner = Scanner::new(r#"var name = "Bob";"#.to_string());
        assert_eq!(
            scanner.source,
            vec![
                'v', 'a', 'r', ' ', 'n', 'a', 'm', 'e', ' ', '=', ' ', '"', 'B', 'o', 'b', '"', ';'
            ]
        );
        assert_eq!(scanner.tokens, vec![]);
        assert_eq!(scanner.start, 0);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.line, 1);
    }

    #[test]
    fn test_scanner_tokens() {
        let scanner = Scanner {
            source: vec![
                'v', 'a', 'r', ' ', 'n', 'a', 'm', 'e', ' ', '=', ' ', '"', 'B', 'o', 'b', '"', ';',
            ],
            tokens: vec![
                Token::new(TokenType::Var, "var", TokenLiteral::Nil, 1),
                Token::new(TokenType::Identifier, "name", TokenLiteral::Nil, 1),
                Token::new(TokenType::Equal, "=", TokenLiteral::Nil, 1),
                Token::new(
                    TokenType::String,
                    "Bob",
                    TokenLiteral::String("Bob".to_string()),
                    1,
                ),
                Token::new(TokenType::Semicolon, ";", TokenLiteral::Nil, 1),
            ],
            start: 0,
            current: 0,
            line: 1,
        };
        assert_eq!(
            scanner.tokens(),
            vec![
                Token::new(TokenType::Var, "var", TokenLiteral::Nil, 1),
                Token::new(TokenType::Identifier, "name", TokenLiteral::Nil, 1),
                Token::new(TokenType::Equal, "=", TokenLiteral::Nil, 1),
                Token::new(
                    TokenType::String,
                    "Bob",
                    TokenLiteral::String("Bob".to_string()),
                    1,
                ),
                Token::new(TokenType::Semicolon, ";", TokenLiteral::Nil, 1),
            ]
        );
    }

    #[test]
    fn test_scan_tokens() {
        let mut scanner = Scanner::new(r#"var name = "Bob";"#.to_string());
        let res = scanner.scan_tokens();
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            vec![
                Token::new(TokenType::Var, "var", TokenLiteral::Nil, 1),
                Token::new(TokenType::Identifier, "name", TokenLiteral::Nil, 1),
                Token::new(TokenType::Equal, "=", TokenLiteral::Nil, 1),
                Token::new(
                    TokenType::String,
                    "Bob",
                    TokenLiteral::String("Bob".to_string()),
                    1,
                ),
                Token::new(TokenType::Semicolon, ";", TokenLiteral::Nil, 1),
                Token::new(TokenType::EOF, "", TokenLiteral::Nil, 1),
            ]
        );

        let mut scanner = Scanner::new(r#"var $age = "five";"#.to_string());
        assert!(scanner.scan_tokens().is_err());
        assert_eq!(
            scanner.tokens(),
            vec![
                Token::new(TokenType::Var, "var", TokenLiteral::Nil, 1),
                Token::new(TokenType::Identifier, "age", TokenLiteral::Nil, 1),
                Token::new(TokenType::Equal, "=", TokenLiteral::Nil, 1),
                Token::new(
                    TokenType::String,
                    "five",
                    TokenLiteral::String("five".to_string()),
                    1,
                ),
                Token::new(TokenType::Semicolon, ";", TokenLiteral::Nil, 1),
                Token::new(TokenType::EOF, "", TokenLiteral::Nil, 1),
            ]
        );
    }

    #[test]
    fn test_is_at_end() {
        let mut scanner = Scanner {
            source: vec![
                'v', 'a', 'r', ' ', 'n', 'a', 'm', 'e', ' ', '=', ' ', '"', 'B', 'o', 'b', '"', ';',
            ],
            tokens: vec![],
            start: 0,
            current: 18,
            line: 1,
        };
        assert!(scanner.is_at_end());
        scanner.current = 10;
        assert!(!scanner.is_at_end());
    }

    #[test]
    fn test_peek() {
        let mut scanner = Scanner {
            source: vec![
                'v', 'a', 'r', ' ', 'n', 'a', 'm', 'e', ' ', '=', ' ', '"', 'B', 'o', 'b', '"', ';',
            ],
            tokens: vec![],
            start: 0,
            current: 18,
            line: 1,
        };
        assert_eq!(scanner.peek(), '\0');
        scanner.current = 10;
        assert_eq!(scanner.peek(), ' ');
    }

    #[test]
    fn test_peek_next() {
        let mut scanner = Scanner {
            source: vec![
                'v', 'a', 'r', ' ', 'n', 'a', 'm', 'e', ' ', '=', ' ', '"', 'B', 'o', 'b', '"', ';',
            ],
            tokens: vec![],
            start: 0,
            current: 18,
            line: 1,
        };
        assert_eq!(scanner.peek_next(), '\0');
        scanner.current = 10;
        assert_eq!(scanner.peek_next(), '"');
    }

    #[test]
    fn test_advance() {
        let mut scanner = Scanner {
            source: vec![
                'v', 'a', 'r', ' ', 'n', 'a', 'm', 'e', ' ', '=', ' ', '"', 'B', 'o', 'b', '"', ';',
            ],
            tokens: vec![],
            start: 0,
            current: 18,
            line: 1,
        };
        assert_eq!(scanner.advance(), None);
        scanner.current = 10;
        assert_eq!(scanner.advance(), Some(' '));
        assert_eq!(scanner.current, 11);
    }

    #[test]
    fn test_conditionally_advance() {
        let mut scanner = Scanner {
            source: vec![
                'v', 'a', 'r', ' ', 'n', 'a', 'm', 'e', ' ', '=', ' ', '"', 'B', 'o', 'b', '"', ';',
            ],
            tokens: vec![],
            start: 0,
            current: 18,
            line: 1,
        };
        assert!(!scanner.conditionally_advance(' '));
        scanner.current = 10;
        assert!(scanner.conditionally_advance(' '));
        assert_eq!(scanner.current, 11);
        assert!(scanner.conditionally_advance('"'));
        assert_eq!(scanner.current, 12);
        assert!(!scanner.conditionally_advance('"'));
        assert_eq!(scanner.current, 12);
    }
}
