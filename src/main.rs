mod error;
mod token_type;
mod token;
mod scanner;
use error::*;
use token_type::*;
use token::*;
use scanner::*;

use std::io::{self, BufRead, BufReader, Read, Stdin};
use std::fs::File;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: jialox [file_path]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()>{
    let buf= std::fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {},
        Err(err) => {
            err.report( "".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    println!("Jialox 0.1.0 | Authored by Jiashu | Finished in June 4 2024");
    print!(">>> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(err) => {
                    err.report("".to_string());
                }   
            }
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<(), JialoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{}", token);
    }
    Ok(())
}



