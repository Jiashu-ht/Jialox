use std::env::args;
use std::io::{self, stdout, BufRead, Write};
use std::rc::Rc;

mod error;
use error::*;

mod token_type;
// use token_type::*;

mod token;
// use token::*;

mod scanner;
use scanner::*;

mod literal;

mod expr;
// use expr::*;

mod parser;
use parser::*;

mod interpreter;
use interpreter::*;

fn main() {
    let args: Vec<String> = args().collect();
    let jialox = Jialox::new();

    if args.len() > 2 {
        println!("Usage: jialox [file_path]");
        std::process::exit(64);
    } else if args.len() == 2 {
        jialox.run_file(&args[1]).expect("Could not run file");
    } else {
        jialox.run_prompt();
    }
}

struct Jialox {
    version: String,
    authored: String,
    finished_time: String,
    interpreter: Interpreter
}

impl Jialox {
    fn new() -> Jialox {
        Jialox {
            version: "0.4.0".to_string(),
            authored: "Jiashu".to_string(),
            finished_time: "June 8 2024".to_string(),
            interpreter: Interpreter::new(),
        }
    }

    fn run_file(&self, path: &String) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        self.print_basic_info();
        if self.run(buf).is_err() {
            // Ignore - error was already reported in run()
            std::process::exit(65);
        }
        Ok(())
    }
    
    fn run_prompt(&self) {
        let stdin = io::stdin();
        self.print_basic_info();
        println!("(Press <Ctrl+z> to exit normally)");
        self.start_input_line();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    self.start_input_line();
                    continue;
                }
                if self.run(line).is_err() {
                    // Ignore - error was already reported in run()
                }
            } else {
                break;
            }
            self.start_input_line();
        }
    }
    
    fn run(&self, source: String) -> Result<(), JialoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
    
        let mut parser = Parser::new(&tokens);
        if let Some(expr) = parser.parse() {
            self.interpreter.interpret(Rc::new(expr));
        }
        Ok(())
    }
    
    fn start_input_line(&self) {
        print!(">>> ");
        stdout().flush().unwrap();
    }
    
    fn print_basic_info(&self) {
        println!(
            "Jialox {} | Authored by {} | Finished in {}",
            self.version,
            self.authored,
            self.finished_time,
        );
    }
    
}