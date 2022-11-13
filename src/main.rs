// mod ast_printer;
mod error;
mod expression;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod statement;
mod token;
mod token_type;

use std::{
    env::args,
    fs::read_to_string,
    io::{self, Write},
};

use error::Error;
use parser::Parser;
use scanner::Scanner;

use crate::interpreter::Interpreter;

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_repl()?,

        2 => run_file(&args[1])?,

        _ => {
            eprintln!("Usage: rlox [script]");
            std::process::exit(64);
        }
    };

    Ok(())
}

fn run_repl() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut line = Default::default();

    println!("welcome to rlox repl");
    loop {
        print!("rlox:> ");
        stdout.flush()?;
        if let Ok(_) = stdin.read_line(&mut line) {
            run(&line).unwrap_or_else(|err| {
                err.report("");
            });
            line.clear();
        } else {
            break;
        }
    }

    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    let source = read_to_string(path)?;

    run(&source).unwrap_or_else(|err| {
        err.report("");
        std::process::exit(65);
    });

    Ok(())
}

fn run(source: &str) -> Result<(), Error> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let statemets = parser.parse()?;
    let interpreter = Interpreter {};
    interpreter.interpret(&statemets)?;

    Ok(())
}
