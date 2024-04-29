use crate::errors::{Error, ErrorKind};
use crate::scanner::Scanner;

/// This represents the context in which the interpreter runs
///
/// It separates different interpreter instances from one another
/// and holds information global to an interpreter.
pub struct Context {
    has_error: bool,
}

impl Context {
    pub fn new() -> Self {
        Self { has_error: false }
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    pub fn set_has_error(&mut self, b: bool) {
        self.has_error = b;
    }
}

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
                if let Err(err) = run(inp) {
                    eprintln!("Failed to run source code with error: {:?}", err);
                }
            }
            Err(err) => {
                eprintln!("Failed to read from interactive shell: {:?}", err);
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
            eprintln!("Failed to read source code as string with error: {:?}", err);
            return Err(Error::new(
                0,
                "",
                "Failed to read source file",
                ErrorKind::IOError,
            ));
        }
    };

    run(prog)?;

    Ok(())
}

/// This starts the compilation process for the source code
fn run(source: String) -> Result<(), Error> {
    println!("PROGRAM: {:?}", source);

    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    tokens.into_iter().for_each(|token| println!("{:?}", token));

    Ok(())
}
