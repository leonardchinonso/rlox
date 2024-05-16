use crate::expressions::{
    assign::Assign,
    binary::Binary,
    expr::{Expr, Visitor},
    grouping::Grouping,
    literal::Literal,
    unary::Unary,
};

pub struct AstPrinter;

impl AstPrinter {
    /// Constructs a new AstPrinter
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    fn parenthesize(&self, name: String, exprs: Vec<Expr>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name.as_str());
        for expr in exprs {
            builder.push(' ');
            builder.push_str(expr.accept(self).as_str());
        }
        builder.push(')');

        builder
    }

    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_assign_expr(&self, _expr: &Assign) -> String {
        unimplemented!()
    }

    fn visit_binary_expr(&self, expr: &Binary) -> String {
        self.parenthesize(expr.operator().lexeme(), vec![expr.left(), expr.right()])
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.parenthesize("group".to_string(), vec![expr.expression()])
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        expr.value().to_string()
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        self.parenthesize(expr.operator().lexeme(), vec![expr.right()])
    }
}
