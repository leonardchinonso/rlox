//! Defines all the statements associated with the Rlox language

pub mod block;
pub mod class;
pub mod expression;
pub mod function;
pub mod if_;
pub mod print;
pub mod return_;
pub mod stmt;
pub mod var;
pub mod while_;

pub use {
    block::Block, class::Class, expression::Expression, function::Function, if_::If, print::Print,
    return_::Return, stmt::Stmt, var::Var, while_::While,
};
