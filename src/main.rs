use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::process;

mod error;
mod scanner;

enum LoxExecutionContext {
    Repl,
    Source(String),
    Error(String),
}

fn get_execution_context() -> LoxExecutionContext {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => LoxExecutionContext::Repl,
        2 => LoxExecutionContext::Source(String::from(&args[1])),
        _ => LoxExecutionContext::Error(String::from("Usage: lox [filepath]")),
    }
}

fn read_source(filename: &String) -> String {
    let path = Path::new(&filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => return s,
    }
}

fn main() {
    match get_execution_context() {
        LoxExecutionContext::Source(filepath) => {
            let source = read_source(&filepath);
            run(source);
            process::exit(0);
        },
        LoxExecutionContext::Repl => {
            run_repl();
        },
        LoxExecutionContext::Error(err) => {
            print!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn run_repl() {
    todo!();
}

fn run(source: String) -> Result<(), error::LoxError> {
    let mut lox_scanner = scanner::Scanner::new(source);
    let tokens = lox_scanner.scan_tokens()?;

    print!("{:?}", tokens);

    Ok(())
}
