use expressions::{binary::Binary, expr::Expr, grouping::Grouping, literal::Literal, unary::Unary};
use lox::token::{Token, TokenLiteral, TokenType};

mod common;
mod expressions;
mod lox;
mod visitors;

fn main() -> Result<(), &'static str> {
    // let args: Vec<_> = std::env::args().collect();
    // match args.len() {
    //     1 => {
    //         let _ = lox::run_prompt();
    //     }
    //     2 => {
    //         let _ = lox::run_file(&args[0]);
    //     }
    //     _ => {
    //         println!("Usage: rlox [script]");
    //         return Err("Incorrect usage");
    //     }
    // }

    let left = Expr::Unary(Unary::new(
        Token::new(TokenType::Minus, "-", TokenLiteral::Nil, 1),
        Expr::Literal(Literal::new(TokenLiteral::Integer(123))),
    ));
    let operator = Token::new(TokenType::Star, "*", TokenLiteral::Nil, 1);
    let right = Expr::Grouping(Grouping::new(Expr::Literal(Literal::new(
        TokenLiteral::Float(45.67),
    ))));

    let expression = Expr::Binary(Binary::new(left, operator, right));
    let ast_printer = visitors::ast_printer::AstPrinter::new();
    let output = ast_printer.print(expression);

    println!("Expression: {:?}", output);

    Ok(())
}
