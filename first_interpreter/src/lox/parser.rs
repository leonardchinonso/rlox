use crate::{
    common::errors::Error,
    expressions::{binary::Binary, expr::Expr, grouping::Grouping, literal::Literal, unary::Unary},
    lox::token::Token,
};

use super::token::{TokenLiteral, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/// Methods for the Parser
impl Parser {
    /// Constructs a new Parser
    pub fn new(tokens: Vec<Token>, current: usize) -> Parser {
        Parser { tokens, current }
    }

    /// sets the tokens for the parser
    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    /// Returns true if any of the token types match the current token
    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Returns true if the current token is of the given type
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind() == token_type
        }
    }

    // Consumes the current token and returns it
    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Returns the current token to consume
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Returns true if at the last token
    fn is_at_end(&self) -> bool {
        self.peek().kind() == TokenType::EOF
    }

    /// Returns the most recently consumed token
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

/// Methods for handling expressions based on precedence
impl Parser {
    /// Returns the equality expression
    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    /// Returns the equality expression
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(vec![TokenType::Bang, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        expr
    }

    /// Returns the comparison expression
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        expr
    }

    /// Returns the terminal expression
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        expr
    }

    /// Returns a factor expression
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        expr
    }

    /// Returns a unary expression
    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Unary::new(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::False]) {
            return Expr::Literal(Literal::new(TokenLiteral::Boolean(false)));
        }
        if self.match_token(vec![TokenType::True]) {
            return Expr::Literal(Literal::new(TokenLiteral::Boolean(true)));
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Expr::Literal(Literal::new(TokenLiteral::Nil));
        }
        if self.match_token(vec![
            TokenType::Integer,
            TokenType::Float,
            TokenType::String,
        ]) {
            return Expr::Literal(Literal::new(self.previous().literal()));
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')' after expression");
            return Expr::Grouping(Grouping::new(expr));
        }
        return Expr::Invalid;
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let token = self.peek();
        let line = token.line();
        Err(Error::report_parse(token, line, message))
    }

    // fn equality(&mut self) -> Expr {
    //     let token_matches = vec![TokenType::Bang, TokenType::EqualEqual];
    //     self.gen(token_matches, self.comparison)
    // }

    // fn gen<F>(&mut self, token_matches: Vec<TokenType>, mut actor: F) -> Expr
    // where
    //     F: FnMut(&mut Self) -> Expr,
    // {
    //     let mut expr = actor(&mut self);
    //     while self.match_token(token_matches) {
    //         let operator = self.previous();
    //         let right = actor(&mut self);
    //         expr = Expr::Binary(Binary::new(expr, operator, right))
    //     }
    //     expr
    // }
}
