use crate::expressions::expr::{Expr, Visitor};
use crate::expressions::literal::Literal;
use crate::rlox::token::{TokenLiteral, TokenType};

use std::any::{Any, TypeId};

struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    /// Evaluates a given expression to a literal
    pub fn evaluate(&self, expr: Expr) -> Box<dyn Any> {
        Box::new(expr.accept(self))
    }
}

impl Visitor<Box<dyn Any>> for Interpreter {
    fn visit_assign_expr(&self, expr: &crate::expressions::assign::Assign) -> Box<dyn Any> {
        todo!()
    }

    fn visit_binary_expr(&self, expr: &crate::expressions::binary::Binary) -> Box<dyn Any> {
        let left = self.evaluate(expr.left());
        let left = *left
            .downcast::<Expr>()
            .expect("unary right should be an expression");

        let right = self.evaluate(expr.right());
        let right = *right
            .downcast::<Expr>()
            .expect("unary right should be an expression");

        match expr.operator().kind() {
            TokenType::Minus => match left {
                Expr::Literal(l) => match l.value() {
                    TokenLiteral::Integer(v1) => {
                        if let Expr::Literal(l) = right {
                            if let TokenLiteral::Integer(v2) = l.value() {
                                return Box::new(v1 - v2);
                            }
                            panic!("cannot apply the MINUS operator on type {:?}", l.value());
                        }
                    }
                    TokenLiteral::Float(v) => Box::new(-v),
                    _ => panic!("cannot apply the MINUS operator on type {:?}", l.value()),
                },
                _ => panic!("cannot apply the MINUS operator on expression {:?}", right),
            },
            TokenType::Bang => match right {
                Expr::Literal(l) => match l.value() {
                    TokenLiteral::Boolean(v) => Box::new(!v),
                    _ => panic!("cannot apply the BANG operator on type {:?}", l.value()),
                },
                _ => panic!("cannot apply the BANG operator on expression {:?}", right),
            },
            // this part of the code is unreachable since MINUS and BANG
            // are the only unary operators
            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(&self, expr: &crate::expressions::grouping::Grouping) -> Box<dyn Any> {
        self.evaluate(expr.expression())
    }

    fn visit_literal_expr(&self, expr: &crate::expressions::literal::Literal) -> Box<dyn Any> {
        Box::new(expr.value())
    }

    fn visit_unary_expr(&self, expr: &crate::expressions::unary::Unary) -> Box<dyn Any> {
        let right = self.evaluate(expr.right());
        let right = *right
            .downcast::<Expr>()
            .expect("unary right should be an expression");

        match expr.operator().kind() {
            TokenType::Minus => match right {
                Expr::Literal(l) => match l.value() {
                    TokenLiteral::Integer(v) => Box::new(-v),
                    TokenLiteral::Float(v) => Box::new(-v),
                    _ => panic!("cannot apply the MINUS operator on type {:?}", l.value()),
                },
                _ => panic!("cannot apply the MINUS operator on expression {:?}", right),
            },
            TokenType::Bang => match right {
                Expr::Literal(l) => match l.value() {
                    TokenLiteral::Boolean(v) => Box::new(!v),
                    _ => panic!("cannot apply the BANG operator on type {:?}", l.value()),
                },
                _ => panic!("cannot apply the BANG operator on expression {:?}", right),
            },
            // this part of the code is unreachable since MINUS and BANG
            // are the only unary operators
            _ => unreachable!(),
        }
    }
}

mod handlers {
    use super::*;

    // Handlers for the different expressions
    fn handle_binary_ops_for_binary_expr(left: Expr, op: TokenType, right: Expr) -> Box<dyn Any> {
        match op {
            TokenType::Minus => match left {
                Expr::Literal(lit) => match lit.value() {
                    TokenLiteral::Integer(l_val) => {
                        match right {
                            Expr::Literal(_) => {
                                match lit.value() {
                                    TokenLiteral::Integer(r_val) => Box::new(l_val - r_val),
                                    _ => {
                                        panic!("cannot subtract {:?} from INTEGER", lit.value());
                                    }
                                }
                            },
                            _ => panic!("")
                        }
                    }
                    TokenLiteral::Float(v) => Box::new(-v),
                    _ => panic!("cannot apply the MINUS operator on type {:?}", l.value()),
                },
                _ => panic!("cannot apply the MINUS operator on expression {:?}", right),
            },
            TokenType::Slash => {}
            TokenType::Star => {}
            _ => {}
        }
    }
}
