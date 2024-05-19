use crate::{
    common::errors::Error,
    expressions::{
        assign::Assign, binary::Binary, expr::Expr, grouping::Grouping, literal::Literal,
        unary::Unary, Variable,
    },
    rlox::token::Token,
    stmt::{Expression, Print, Stmt, Var},
};

use super::token::{TokenLiteral, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/// Methods for the Parser
impl Parser {
    /// Constructs a new Parser
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    /// Parses a series of statements, as many as it
    /// can find until it hits the end of the input
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
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
        if !self.is_at_end() {
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

/// Private methods for handling statements
impl Parser {
    /// Parses a series of statements when called repeatedly
    fn declaration(&mut self) -> Result<Stmt, Error> {
        if self.match_token(vec![TokenType::Var]) {
            match self.var_declaration() {
                Ok(v) => return Ok(v),
                Err(err) => {
                    // skip further tokens in this statement and
                    // consume the next tokens
                    self.synchronize();
                    return Err(err);
                }
            }
        }

        match self.statement() {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                // skip further tokens in this statement and
                // consume the next tokens
                self.synchronize();
                return Err(err);
            }
        }
    }

    /// Parses a single statement
    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    /// Parses the print statement
    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Print(Print::new(value)))
    }

    /// Parses an expression statement
    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::Expression(Expression::new(expr)))
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, "Expected variable name.")?;

        let mut initializer = None;
        if self.match_token(vec![TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;
        Ok(Stmt::Var(Var::new(name, initializer)))
    }
}

/// Private methods for handling expressions based on precedence
impl Parser {
    /// Returns the equality expression
    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    /// Figure out if the statement is an assignment
    /// or an expression.
    /// Returns an expression of the specific type
    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.equality()?;

        if self.match_token(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(v) = expr {
                return Ok(Expr::Assign(Assign::new(v.name().clone(), value)));
            }

            return Err(Error::report_parse(equals, "Invalid assignment target."));
        }

        return Ok(expr);
    }

    /// Returns the equality expression
    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    /// Returns the comparison expression
    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        Ok(expr)
    }

    /// Returns the terminal expression
    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        Ok(expr)
    }

    /// Returns a factor expression
    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }
        Ok(expr)
    }

    /// Returns a unary expression
    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary::new(operator, right)));
        }
        self.primary()
    }

    /// Returns a primary expression
    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::new(TokenLiteral::Boolean(false))));
        }
        if self.match_token(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::new(TokenLiteral::Boolean(true))));
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::new(TokenLiteral::Nil)));
        }
        if self.match_token(vec![
            TokenType::Integer,
            TokenType::Float,
            TokenType::String,
        ]) {
            return Ok(Expr::Literal(Literal::new(self.previous().literal())));
        }
        if self.match_token(vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(Variable::new(self.previous())));
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping(Grouping::new(expr)));
        }
        Err(Error::report_parse(self.peek(), "Expected expression."))
    }

    /// Consumes a token at the current position if it is the correct token
    ///
    /// Returns a ParseError if the token is incorrect
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(Error::report_parse(self.peek(), message))
    }

    /// Discards tokens until it finds a statement boundary
    ///
    /// Usually used after a parse error occurs in order to find the next valid statements
    fn synchronize(&mut self) {
        // advance the current pointer to consume the invalid token
        self.advance();

        while self.is_at_end() {
            // this is a statement boundary
            if self.previous().kind() == TokenType::Semicolon {
                return;
            }

            // match only tokens that can start a valid statement
            // discarding the rest
            match self.peek().kind() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::For
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => return,
                _ => {}
            }

            self.advance();
        }
    }
}
