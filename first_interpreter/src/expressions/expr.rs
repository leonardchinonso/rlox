use super::{assign::Assign, binary::Binary, grouping::Grouping, literal::Literal, unary::Unary};

/// Trait for a structure implementing all the methods to
/// handle different expressions
pub trait Visitor<R> {
    fn visit_assign_expr(&self, expr: &Assign) -> R;
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    // fn visit_call_expr(&self, expr: &Call) -> R;
    // fn visit_get_expr(&self, expr: &Get) -> R;
    fn visit_grouping_expr(&self, expr: &Grouping) -> R;
    fn visit_literal_expr(&self, expr: &Literal) -> R;
    // fn visit_logical_expr(&self, expr: &Logical) -> R;
    // fn visit_set_expr(&self, expr: &Set) -> R;
    // fn visit_super_expr(&self, expr: &Super) -> R;
    // fn visit_this_expr(&self, expr: &This) -> R;
    fn visit_unary_expr(&self, expr: &Unary) -> R;
    // fn visit_variable_expr(&self, expr: &Variable) -> R;
}

/// Represents all forms of expressions
#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
}

impl Expr {
    /// Accepts the visitor structure to perform an expression
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Assign(expr) => visitor.visit_assign_expr(expr),
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
        }
    }
}
