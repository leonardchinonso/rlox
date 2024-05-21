use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::common::errors::Error;
use crate::expressions::expr::{Expr, Visitor as ExprVisitor};
use crate::rlox::token::{TokenLiteral, TokenType};
use crate::stmt::stmt::Visitor as StmtVisitor;
use crate::stmt::{Block, Stmt};

use super::environment::Environment;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    /// Begins the interpretation and evaluation process
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), Error> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    /// Executes a given statement
    pub fn execute(&mut self, stmt: Stmt) -> Result<(), Error> {
        stmt.accept(self)
    }

    /// Evaluates a given expression to a literal
    pub fn evaluate(&mut self, expr: Expr) -> Result<TokenLiteral, Error> {
        Ok(expr.accept(self)?)
    }

    /// Executes a list of statements in the context of the given environment
    fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), Error> {
        // replace the interpreter's environment with the one from the context
        // this is so that the statements are executed with their scopes in view
        let previous = std::mem::replace(&mut self.environment, environment);
        for statement in statements {
            // if there is an error executing a statement, switch to the original
            // scope before terminating the execution pipeline
            if let Err(err) = self.execute(statement) {
                self.environment = previous;
                return Err(err);
            };
        }
        // set it back to the original environment
        self.environment = previous;
        Ok(())
    }
}

impl ExprVisitor<Result<TokenLiteral, Error>> for Interpreter {
    fn visit_assign_expr(
        &mut self,
        expr: &crate::expressions::assign::Assign,
    ) -> Result<TokenLiteral, Error> {
        let value = self.evaluate(expr.value())?;
        self.environment
            .borrow_mut()
            .assign(expr.name(), value.clone())?;
        Ok(value)
    }

    fn visit_binary_expr(
        &mut self,
        expr: &crate::expressions::binary::Binary,
    ) -> Result<TokenLiteral, Error> {
        let left = self.evaluate(expr.left())?;
        let right = self.evaluate(expr.right())?;

        match expr.operator().kind() {
            TokenType::Minus => match left {
                TokenLiteral::Integer(l_val) => {
                    if let TokenLiteral::Integer(r_val) = right {
                        return Ok(TokenLiteral::Integer(l_val - r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("MINUS", None),
                    ))
                }
                TokenLiteral::Float(l_val) => {
                    if let TokenLiteral::Float(r_val) = right {
                        return Ok(TokenLiteral::Float(l_val - r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("MINUS", None),
                    ))
                }
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    &get_runtime_err_msg("MINUS", Some("numeric")),
                )),
            },
            TokenType::Slash => match left {
                TokenLiteral::Integer(l_val) => {
                    if let TokenLiteral::Integer(r_val) = right {
                        if r_val == 0 {
                            panic!("division by zero not allowed!");
                        }
                        return Ok(TokenLiteral::Integer(l_val / r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("DIVISION", None),
                    ))
                }
                TokenLiteral::Float(l_val) => {
                    if let TokenLiteral::Float(r_val) = right {
                        if r_val == 0f64 {
                            panic!("division by zero not allowed!");
                        }
                        return Ok(TokenLiteral::Float(l_val / r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("DIVISION", None),
                    ))
                }
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    &get_runtime_err_msg("DIVISION", Some("numeric")),
                )),
            },
            TokenType::Star => match left {
                TokenLiteral::Integer(l_val) => {
                    if let TokenLiteral::Integer(r_val) = right {
                        return Ok(TokenLiteral::Integer(l_val * r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("PRODUCT", None),
                    ))
                }
                TokenLiteral::Float(l_val) => {
                    if let TokenLiteral::Float(r_val) = right {
                        return Ok(TokenLiteral::Float(l_val * r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("PRODUCT", None),
                    ))
                }
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    &get_runtime_err_msg("PRODUCT", Some("numeric")),
                )),
            },
            TokenType::Plus => match left {
                TokenLiteral::Integer(l_val) => {
                    if let TokenLiteral::Integer(r_val) = right {
                        return Ok(TokenLiteral::Integer(l_val + r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("ADD", None),
                    ))
                }
                TokenLiteral::Float(l_val) => {
                    if let TokenLiteral::Float(r_val) = right {
                        return Ok(TokenLiteral::Float(l_val + r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("ADD", None),
                    ))
                }
                TokenLiteral::String(mut l_val) => {
                    if let TokenLiteral::String(r_val) = right {
                        l_val.push_str(&r_val);
                        return Ok(TokenLiteral::String(l_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("ADD", None),
                    ))
                }
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    &get_runtime_err_msg("ADD", Some("both numeric or both string")),
                )),
            },
            TokenType::Greater => match left {
                TokenLiteral::Integer(l_val) => {
                    if let TokenLiteral::Integer(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val > r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("GREATER THAN", None),
                    ))
                }
                TokenLiteral::Float(l_val) => {
                    if let TokenLiteral::Float(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val > r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("GREATER THAN", None),
                    ))
                }
                TokenLiteral::String(l_val) => {
                    if let TokenLiteral::String(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val > r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("GREATER THAN", None),
                    ))
                }
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    &get_runtime_err_msg("GREATER THAN", Some("valid")),
                )),
            },
            TokenType::GreaterEqual => match left {
                TokenLiteral::Integer(l_val) => {
                    if let TokenLiteral::Integer(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val >= r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("GREATER THAN OR EQUAL", None),
                    ))
                }
                TokenLiteral::Float(l_val) => {
                    if let TokenLiteral::Float(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val >= r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("GREATER THAN OR EQUAL", None),
                    ))
                }
                TokenLiteral::String(l_val) => {
                    if let TokenLiteral::String(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val >= r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("GREATER THAN OR EQUAL", None),
                    ))
                }
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    &get_runtime_err_msg("GREATER THAN OR EQUAL", Some("valid")),
                )),
            },
            TokenType::Less => match left {
                TokenLiteral::Integer(l_val) => {
                    if let TokenLiteral::Integer(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val < r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("LESS THAN", None),
                    ))
                }
                TokenLiteral::Float(l_val) => {
                    if let TokenLiteral::Float(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val < r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("LESS THAN", None),
                    ))
                }
                TokenLiteral::String(l_val) => {
                    if let TokenLiteral::String(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val < r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("LESS THAN", None),
                    ))
                }
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    &get_runtime_err_msg("LESS THAN", Some("valid")),
                )),
            },
            TokenType::LessEqual => match left {
                TokenLiteral::Integer(l_val) => {
                    if let TokenLiteral::Integer(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val <= r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("LESS THAN OR EQUAL", None),
                    ))
                }
                TokenLiteral::Float(l_val) => {
                    if let TokenLiteral::Float(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val <= r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("LESS THAN OR EQUAL", None),
                    ))
                }
                TokenLiteral::String(l_val) => {
                    if let TokenLiteral::String(r_val) = right {
                        return Ok(TokenLiteral::Boolean(l_val <= r_val));
                    }
                    Err(Error::report_runtime(
                        expr.operator().clone(),
                        &get_runtime_err_msg("LESS THAN OR EQUAL", None),
                    ))
                }
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    &get_runtime_err_msg("LESS THAN OR EQUAL", Some("valid")),
                )),
            },
            TokenType::BangEqual => match left {
                // Instead of panicking for distinct types, return false.
                // The language should allow equality comparison for distinct types.
                TokenLiteral::Integer(l_val) => match right {
                    TokenLiteral::Integer(r_val) => Ok(TokenLiteral::Boolean(l_val != r_val)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
                TokenLiteral::Float(l_val) => match right {
                    TokenLiteral::Float(r_val) => Ok(TokenLiteral::Boolean(l_val != r_val)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
                TokenLiteral::String(l_val) => match right {
                    TokenLiteral::String(r_val) => Ok(TokenLiteral::Boolean(l_val != r_val)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
                TokenLiteral::Boolean(l_val) => match right {
                    TokenLiteral::Boolean(r_val) => Ok(TokenLiteral::Boolean(l_val != r_val)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
                TokenLiteral::Nil => match right {
                    TokenLiteral::Nil => Ok(TokenLiteral::Boolean(true)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
            },
            TokenType::EqualEqual => match left {
                // Instead of panicking for distinct types, return false.
                // The language should allow equality comparison for distinct types.
                TokenLiteral::Integer(l_val) => match right {
                    TokenLiteral::Integer(r_val) => Ok(TokenLiteral::Boolean(l_val == r_val)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
                TokenLiteral::Float(l_val) => match right {
                    TokenLiteral::Float(r_val) => Ok(TokenLiteral::Boolean(l_val == r_val)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
                TokenLiteral::String(l_val) => match right {
                    TokenLiteral::String(r_val) => Ok(TokenLiteral::Boolean(l_val == r_val)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
                TokenLiteral::Boolean(l_val) => match right {
                    TokenLiteral::Boolean(r_val) => Ok(TokenLiteral::Boolean(l_val == r_val)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
                TokenLiteral::Nil => match right {
                    TokenLiteral::Nil => Ok(TokenLiteral::Boolean(true)),
                    _ => Ok(TokenLiteral::Boolean(false)),
                },
            },
            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &crate::expressions::grouping::Grouping,
    ) -> Result<TokenLiteral, Error> {
        Ok(self.evaluate(expr.expression())?)
    }

    fn visit_literal_expr(
        &mut self,
        expr: &crate::expressions::literal::Literal,
    ) -> Result<TokenLiteral, Error> {
        Ok(expr.value())
    }

    fn visit_unary_expr(
        &mut self,
        expr: &crate::expressions::unary::Unary,
    ) -> Result<TokenLiteral, Error> {
        let right = self.evaluate(expr.right())?;

        match expr.operator().kind() {
            TokenType::Minus => match right {
                TokenLiteral::Integer(v) => Ok(TokenLiteral::Integer(-v)),
                TokenLiteral::Float(v) => Ok(TokenLiteral::Float(-v)),
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    "MINUS must have a numeric operand",
                )),
            },
            TokenType::Bang => match right {
                TokenLiteral::Boolean(v) => Ok(TokenLiteral::Boolean(!v)),
                TokenLiteral::Nil => Ok(TokenLiteral::Boolean(true)),
                _ => Err(Error::report_runtime(
                    expr.operator().clone(),
                    "NEGATION must have a valid operand",
                )),
            },
            // this part of the code is unreachable since MINUS and BANG
            // are the only unary operators
            _ => unreachable!(),
        }
    }

    fn visit_variable_expr(
        &mut self,
        expr: &crate::expressions::Variable,
    ) -> Result<TokenLiteral, Error> {
        self.environment.borrow().get(&expr.name())
    }

    fn visit_logical_expr(
        &mut self,
        expr: &crate::expressions::Logical,
    ) -> Result<TokenLiteral, Error> {
        let left = self.evaluate(expr.left())?;

        // short circuit the boolean operation if its OR or AND
        if expr.operator().kind() == TokenType::Or {
            match left {
                TokenLiteral::Boolean(is_true) => match is_true {
                    true => return Ok(left), // return true if one is true
                    false => {
                        let r_bool = self.evaluate(expr.right())?;
                        if matches!(r_bool, TokenLiteral::Boolean(_)) {
                            return Ok(r_bool);
                        }
                        return Err(Error::report_generic(
                            "Right operand must be a boolean value",
                        ));
                    }
                },
                _ => {
                    return Err(Error::report_generic(
                        "Left operand must be a boolean value",
                    ))
                }
            }
        } else if expr.operator().kind() == TokenType::And {
            match left {
                TokenLiteral::Boolean(is_true) => match is_true {
                    true => {
                        // only return true if both are true
                        let r_bool = self.evaluate(expr.right())?;
                        if matches!(r_bool, TokenLiteral::Boolean(_)) {
                            return Ok(r_bool);
                        }
                        return Err(Error::report_generic(
                            "Right operand must be a boolean value",
                        ));
                    }
                    false => return Ok(left),
                },
                _ => {
                    return Err(Error::report_generic(
                        "Left operand must be a boolean value",
                    ))
                }
            }
        }

        unreachable!()
    }
}

impl StmtVisitor<Result<(), Error>> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &crate::stmt::Block) -> Result<(), Error> {
        let parent_env = Environment::with_parent(self.environment.clone());
        self.execute_block(stmt.statements(), Rc::new(RefCell::new(parent_env)))?;
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &crate::stmt::Class) -> Result<(), Error> {
        todo!()
    }

    fn visit_expression_stmt(&mut self, stmt: &crate::stmt::Expression) -> Result<(), Error> {
        self.evaluate(stmt.expression())?;
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &crate::stmt::Function) -> Result<(), Error> {
        todo!()
    }

    fn visit_if_stmt(&mut self, stmt: &crate::stmt::If) -> Result<(), Error> {
        let value = self.evaluate(stmt.condition())?;
        if let TokenLiteral::Boolean(cond_is_true) = value {
            if cond_is_true {
                self.execute(stmt.then_branch())?;
                return Ok(());
            } else if stmt.else_branch().is_some() {
                self.execute(stmt.else_branch().unwrap())?;
                return Ok(());
            }
            return Ok(());
        }
        Err(Error::report_generic(
            "Condition in if statement must evaluate to 'true' or 'false'",
        ))
    }

    fn visit_print_stmt(&mut self, stmt: &crate::stmt::Print) -> Result<(), Error> {
        let value = self.evaluate(stmt.expression())?;
        value.print();
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &crate::stmt::Return) -> Result<(), Error> {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &crate::stmt::Var) -> Result<(), Error> {
        let value = self.evaluate(stmt.initializer())?;
        self.environment
            .borrow_mut()
            .define(stmt.name().lexeme(), value);
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &crate::stmt::While) -> Result<(), Error> {
        loop {
            let value = self.evaluate(stmt.condition())?;
            if let TokenLiteral::Boolean(bool_value) = value {
                if bool_value {
                    self.execute(*stmt.body())?;
                } else {
                    break;
                }
            } else {
                return Err(Error::report_generic(
                    "Condition in if statement must evaluate to 'true' or 'false'",
                ));
            }
        }

        Ok(())
    }
}

fn get_runtime_err_msg(operator: &str, actor: Option<&str>) -> String {
    if actor.is_some() {
        format!("{} must have {} operands", operator, actor.unwrap())
    } else {
        format!("Cannot use {} on two distinct types", operator)
    }
}
