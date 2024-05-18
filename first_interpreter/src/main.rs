mod common;
mod expressions;
mod stmt;
mod rlox;
mod visitors;

fn main() -> Result<(), &'static str> {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        1 => {
            let _ = rlox::run_prompt();
        }
        2 => {
            let _ = rlox::run_file(&args[1]);
        }
        _ => {
            println!("Usage: rlox [script]");
            return Err("Incorrect usage");
        }
    }

    Ok(())
}
