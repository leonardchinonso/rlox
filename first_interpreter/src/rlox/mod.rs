pub mod callable;
pub mod environment;
pub mod interpreter;
pub mod native;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod types;

use crate::{common::errors::Error, rlox::interpreter::Interpreter};
use parser::Parser;
use scanner::Scanner;

pub use {
    callable::RloxCallable,
    native::{ClockFunction, NativeCallable},
    token::{Token, TokenLiteral},
    types::Value,
};

/// This is a wrapper for running the source code
///
/// It starts the interpreter process on every line read by the shell.
pub fn run_prompt() -> Result<(), &'static str> {
    println!("Welcome to the Lox interactive shell!");
    loop {
        print!(">> ");
        let mut inp = String::new();
        match std::io::stdin().read_line(&mut inp) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let _ = run(inp);
                // if let Err(err) = run(inp) {
                //     eprintln!("Execution error: {:?}", err);
                // };
            }
            Err(err) => {
                eprintln!("Failed to read from interactive shell: {:?}", err);
                // stop the interactive session
                return Err("Error reading from interactive shell");
            }
        }
    }

    Ok(())
}

/// This is a wrapper for running the source code
///
/// It starts the interpreter process after reading the source file
pub fn run_file(file_path: &str) -> Result<(), Error> {
    let prog = match std::fs::read_to_string(file_path) {
        Ok(prog) => prog,
        Err(err) => {
            return Err(Error::report_io(&format!(
                "Failed to read source file: {:?}",
                err
            )));
        }
    };

    run(prog)?;

    Ok(())
}

/// This starts the compilation process for the source code
fn run(source: String) -> Result<(), Error> {
    println!("Running program...");

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e() {
        let test_cases = [
            ("(4 + 3);", "(group (+ 4 3))"),
            (
                "(4 + 3 * 12) - (7 / 5) == 13;",
                "(== (- (group (+ 4 (* 3 12))) (group (/ 7 5))) 13)",
            ),
            (
                "4 + 3 * 12 - 7 / 5 == 13;",
                "(== (- (+ 4 (* 3 12)) (/ 7 5)) 13)",
            ),
        ];

        for (inp, _) in test_cases {
            let mut scanner = Scanner::new(inp.to_string());

            let res = scanner.scan_tokens();
            assert!(res.is_ok());

            let tokens = res.unwrap();

            let mut parser = Parser::new(tokens);
            let parsed_result = parser.parse();
            assert!(parsed_result.is_ok());
        }
    }
}
