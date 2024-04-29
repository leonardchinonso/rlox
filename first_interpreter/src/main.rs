mod errors;
mod lox;
mod scanner;
mod token;

fn main() -> Result<(), &'static str> {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        1 => {
            let _ = lox::run_prompt();
        }
        2 => {
            let _ = lox::run_file(&args[0]);
        }
        _ => {
            println!("Usage: rlox [script]");
            return Err("Incorrect usage");
        }
    }

    Ok(())
}
