use crate::stmt::{Block, Class, Expression, Function, If, Print, Return, Var, While};

/// Trait for a structure implementing all the methods to
/// handle different statements
pub trait Visitor<R> {
    fn visit_block_stmt(&self, stmt: &Block) -> R;
    fn visit_class_stmt(&self, stmt: &Class) -> R;
    fn visit_expression_stmt(&self, stmt: &Expression) -> R;
    fn visit_function_stmt(&self, stmt: &Function) -> R;
    fn visit_if_stmt(&self, stmt: &If) -> R;
    fn visit_print_stmt(&self, stmt: &Print) -> R;
    fn visit_return_stmt(&self, stmt: &Return) -> R;
    fn visit_var_stmt(&mut self, stmt: &Var) -> R;
    fn visit_while_stmt(&self, stmt: &While) -> R;
}

/// Represents all forms of statements
#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Block),
    Class(Class),
    Expression(Expression),
    Function(Function),
    If(If),
    Print(Print),
    Return(Return),
    Var(Var),
    While(While),
}

impl Stmt {
    /// Accepts the visitor structure to perform a statement
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::Class(stmt) => visitor.visit_class_stmt(stmt),
            Stmt::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::Function(stmt) => visitor.visit_function_stmt(stmt),
            Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
            Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::Return(stmt) => visitor.visit_return_stmt(stmt),
            Stmt::Var(stmt) => visitor.visit_var_stmt(stmt),
            Stmt::While(stmt) => visitor.visit_while_stmt(stmt),
        }
    }
}
