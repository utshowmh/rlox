pub struct Error {
    line: usize,
    message: String,
}

impl Error {
    pub fn error(line: usize, message: &str) -> Error {
        Error {
            line,
            message: message.to_string(),
        }
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
    }
}
