//! Contains methods and types for variable resolution.
//! This module runs after the Parser has produced the AST
//! and before the Interpreter begins execution.
//!
//! There are no side effects produced from this module.
//! All states remain the same after this module runs.

use std::collections::HashMap;

use crate::{
    common::{Error, Stack},
    expressions::{expr::Visitor as ExprVisitor, Expr, Literal},
    rlox::{Interpreter, Token, TokenLiteral},
    stmt::{stmt::Visitor as StmtVisitor, Stmt},
};

/// Represents a structure for handling variable resolution
struct Resolver {
    interpreter: Interpreter,
    /// The boolean value represents whether or not the variable
    /// initializer has been resolved
    scopes: Stack<HashMap<String, bool>>,
}

impl Resolver {
    /// Constructs a new Resolver
    pub fn new(interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Stack::new(),
        }
    }

    /// Resolves a list of statements
    fn resolve_statements(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.resolve_statement(statement)
        }
    }

    /// Resolves a statement
    fn resolve_statement(&mut self, stmt: Stmt) {
        stmt.accept(self);
    }

    /// Resolves an expression
    fn resolve_expression(&mut self, expr: Expr) {
        expr.accept(self);
    }

    /// Creates a new block scope
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Exits a block scope
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    /// Adds the variable to the innermost scope so that
    /// it shadows any outer scope.
    fn declare(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self
            .scopes
            .peek_mut()
            .expect("should not happen, scope should exist.");
        scope.insert(name.lexeme(), false);
    }

    /// Resolve the initializer expression of a variable by defining it
    fn define(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self
            .scopes
            .peek_mut()
            .expect("should not happen, scope should exist.");
        scope.entry(name.lexeme()).and_modify(|v| *v = true);
    }

    /// Resolves a local variable
    fn resolve_local(&self, expr: Expr, name: Token) {
        for i in (0..self.scopes.len()).rev() {
            if self
                .scopes
                .get(i)
                .expect("index i to retrieve for scope should be valid")
                .contains_key(&name.lexeme())
            {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
            }
        }
    }
}

impl ExprVisitor<Result<(), Error>> for Resolver {
    fn visit_assign_expr(&mut self, expr: &crate::expressions::Assign) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_binary_expr(&mut self, expr: &crate::expressions::Binary) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_call_expr(&mut self, expr: &crate::expressions::Call) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_grouping_expr(&mut self, expr: &crate::expressions::Grouping) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_literal_expr(&mut self, expr: &crate::expressions::Literal) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_logical_expr(&mut self, expr: &crate::expressions::Logical) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_unary_expr(&mut self, expr: &crate::expressions::Unary) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_variable_expr(&mut self, expr: &crate::expressions::Variable) -> Result<(), Error> {
        if !self.scopes.is_empty()
            && matches!(
                self.scopes
                    .peek()
                    .expect("should not happen, scope should exist.")
                    .get(&expr.name().lexeme()),
                Some(&false)
            )
        {
            return Err(Error::report_parse(
                expr.name().clone(),
                "Cannot read local variable in its own initializer",
            ));
        }
        self.resolve_local(expr, expr.name());
        Ok(())
    }
}

impl StmtVisitor<Result<(), Error>> for Resolver {
    fn visit_block_stmt(&mut self, stmt: &crate::stmt::Block) -> Result<(), Error> {
        self.begin_scope();
        self.resolve_statements(stmt.statements());
        self.end_scope();
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &crate::stmt::Class) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_expression_stmt(&mut self, stmt: &crate::stmt::Expression) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_function_stmt(&mut self, stmt: &crate::stmt::Function) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_if_stmt(&mut self, stmt: &crate::stmt::If) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_print_stmt(&mut self, stmt: &crate::stmt::Print) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_return_stmt(&mut self, stmt: &crate::stmt::Return) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_var_stmt(&mut self, stmt: &crate::stmt::Var) -> Result<(), Error> {
        self.declare(stmt.name());
        if stmt.is_initialized() {
            self.resolve_expression(stmt.initializer())
        }
        self.define(stmt.name());
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &crate::stmt::While) -> Result<(), Error> {
        unimplemented!()
    }
}
