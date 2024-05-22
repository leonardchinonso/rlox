use crate::{
    expressions::{
        assign::Assign,
        binary::Binary,
        expr::{Expr, Visitor},
        grouping::Grouping,
        literal::Literal,
        unary::Unary,
    },
    rlox::TokenLiteral,
};

/// Represents a printer for the abstract syntax tree
pub struct AstPrinter;

impl AstPrinter {
    /// Constructs a new AstPrinter
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    /// Surrounds the given expression in paratheses
    fn parenthesize(&mut self, name: String, exprs: Vec<Expr>) -> String {
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

    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }
}

/// Implements the Visitor trait for AstPrinter
impl Visitor<String> for AstPrinter {
    fn visit_assign_expr(&mut self, _expr: &Assign) -> String {
        unimplemented!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        self.parenthesize(expr.operator().lexeme(), vec![expr.left(), expr.right()])
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group".to_string(), vec![expr.expression()])
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> String {
        expr.value()
            .owned::<TokenLiteral>()
            .expect("Must be a valid literal")
            .to_string()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        self.parenthesize(expr.operator().lexeme(), vec![expr.right()])
    }

    fn visit_variable_expr(&mut self, _expr: &crate::expressions::Variable) -> String {
        unimplemented!()
    }

    fn visit_logical_expr(&mut self, _expr: &crate::expressions::Logical) -> String {
        unimplemented!()
    }

    fn visit_call_expr(&mut self, expr: &crate::expressions::Call) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::rlox::{
        token::{Token, TokenLiteral, TokenType},
        Value,
    };

    use super::*;

    #[test]
    fn test_ast_printer() {
        let left = Expr::Unary(Unary::new(
            Token::new(TokenType::Minus, "-", TokenLiteral::Nil, 1),
            Expr::Literal(Literal::new(Value::new(TokenLiteral::Integer(123)))),
        ));
        let operator = Token::new(TokenType::Star, "*", TokenLiteral::Nil, 1);
        let right = Expr::Grouping(Grouping::new(Expr::Literal(Literal::new(Value::new(
            TokenLiteral::Float(45.67),
        )))));

        let expression = Expr::Binary(Binary::new(left, operator, right));
        let mut ast_printer = AstPrinter::new();
        let output = ast_printer.print(expression);

        assert_eq!("(* (- 123) (group 45.67))".to_string(), output);
    }
}
