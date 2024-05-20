use crate::expressions::{Assign, Binary, Grouping, Literal, Logical, Unary, Variable};

/// Trait for a structure implementing all the methods to
/// handle different expressions
pub trait Visitor<R> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> R;
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    // fn visit_call_expr(&mut self, expr: &Call) -> R;
    // fn visit_get_expr(&mut self, expr: &Get) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literal_expr(&mut self, expr: &Literal) -> R;
    fn visit_logical_expr(&mut self, expr: &Logical) -> R;
    // fn visit_set_expr(&mut self, expr: &Set) -> R;
    // fn visit_super_expr(&mut self, expr: &Super) -> R;
    // fn visit_this_expr(&mut self, expr: &This) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
    fn visit_variable_expr(&mut self, expr: &Variable) -> R;
}

/// Represents all forms of expressions using wrappers
#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Variable(Variable),
    Logical(Logical),
}

impl Expr {
    /// Accepts the visitor structure to perform an expression
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
            Expr::Variable(expr) => visitor.visit_variable_expr(expr),
            Expr::Logical(expr) => visitor.visit_logical_expr(expr),
        }
    }
}
