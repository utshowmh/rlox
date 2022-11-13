use std::fmt;

pub enum ErrorType {
    LexingError,
    ParsingError,
    RuntimeError,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::LexingError => write!(f, "LexingError"),
            ErrorType::ParsingError => write!(f, "ParsingError"),
            ErrorType::RuntimeError => write!(f, "RuntimeError"),
        }
    }
}

pub struct Error {
    line: usize,
    etype: ErrorType,
    message: String,
}

impl Error {
    pub fn new(line: usize, etype: ErrorType, message: &str) -> Error {
        Error {
            line,
            etype,
            message: message.to_string(),
        }
    }

    pub fn report(&self, loc: &str) {
        eprintln!(
            "[line {}] {}{}: {}",
            self.line, self.etype, loc, self.message
        );
    }
}
