mod error;
mod object;
mod scanner;
mod token;
mod token_type;

use std::{
    env::args,
    fs::read_to_string,
    io::{self, Write},
};

use error::Error;
use scanner::Scanner;

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt()?,

        2 => run_file(&args[1])?,

        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    };

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut line = Default::default();

    loop {
        print!("rlox :> ");
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

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
