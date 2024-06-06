use std::io::{self, stdout, BufRead, Write};
use std::env::args;

mod error;
use error::*;

mod token_type;
mod token;

mod scanner;
use scanner::*;

mod literal;

mod expr;
//use expr::*;

// use token_type::*;
// use token::*;


fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: jialox [file_path]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()>{
    let buf= std::fs::read_to_string(path)?;
    print_basic_info();
    if run(buf).is_err() {
        // Ignore - error was already reported in run()
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print_basic_info();
    println!("(Press <Ctrl+z> to exit normally)");
    start_input_line();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                start_input_line();
                continue;
            }
            if run(line).is_err() {
                // Ignore - error was already reported in run()
            }
        } else {
            break;
        }
        start_input_line();
    }
}

fn run(source: String) -> Result<(), JialoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn start_input_line() {
    print!(">>> ");
    stdout().flush().unwrap();
}

fn print_basic_info() {
    println!("Jialox 0.1.0 | Authored by Jiashu | Finished in June 5 2024");
}