use crate::errors::Error;
use crate::scanner::Scanner;

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
            return Err(Error::report_io(
                None,
                &format!("Failed to read source file: {:?}", err),
            ));
        }
    };

    run(prog)?;

    Ok(())
}

/// This starts the compilation process for the source code
fn run(source: String) -> Result<(), Error> {
    println!("PROGRAM: {:?}", source);

    let mut scanner = Scanner::new(source);
    scanner.scan_tokens()?;

    scanner
        .tokens()
        .into_iter()
        .for_each(|token| println!("{:?}", token));

    Ok(())
}
