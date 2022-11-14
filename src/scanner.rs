use std::collections::HashMap;

use crate::{
    error::{Error, ErrorType},
    object::Object,
    token::Token,
    token_type::TokenType,
};

pub struct Scanner {
    source: String,
    source_as_vec: Vec<u8>,
    source_len: usize,

    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            source_as_vec: source.as_bytes().to_vec(),

            source_len: source.len(),
            keywords: HashMap::new(),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        self.init_keywords();

        while !self.is_eof() {
            self.scan_token()?;
            self.start = self.current;
        }
        self.add_token_without_literal(TokenType::EOF);

        Ok(self.tokens.clone())
    }
}

impl Scanner {
    fn scan_token(&mut self) -> Result<(), Error> {
        let current_charecter = self.advance();

        match current_charecter {
            ' ' | '\r' | '\t' => return Ok(()),

            '\n' => self.line += 1,

            '(' => self.add_token_without_literal(TokenType::LeftParen),

            ')' => self.add_token_without_literal(TokenType::RightParen),

            '{' => self.add_token_without_literal(TokenType::LeftBrace),

            '}' => self.add_token_without_literal(TokenType::RightBrace),

            ';' => self.add_token_without_literal(TokenType::Semicolon),

            ',' => self.add_token_without_literal(TokenType::Comma),

            '.' => self.add_token_without_literal(TokenType::Dot),

            '+' => self.add_token_without_literal(TokenType::Plus),

            '-' => self.add_token_without_literal(TokenType::Minus),

            '*' => self.add_token_without_literal(TokenType::Star),

            '/' => {
                if self.does_match('/') {
                    self.comment();
                } else if self.does_match('*') {
                    self.multiline_comment()?;
                } else {
                    self.add_token_without_literal(TokenType::Slash);
                }
            }

            '!' => {
                if self.does_match('=') {
                    self.add_token_without_literal(TokenType::BangEqual);
                } else {
                    self.add_token_without_literal(TokenType::Bang);
                }
            }

            '=' => {
                if self.does_match('=') {
                    self.add_token_without_literal(TokenType::EqualEqual);
                } else {
                    self.add_token_without_literal(TokenType::Equal);
                }
            }

            '<' => {
                if self.does_match('=') {
                    self.add_token_without_literal(TokenType::LessEqual);
                } else {
                    self.add_token_without_literal(TokenType::Less);
                }
            }

            '>' => {
                if self.does_match('=') {
                    self.add_token_without_literal(TokenType::GreaterEqual);
                } else {
                    self.add_token_without_literal(TokenType::Greater);
                }
            }

            '"' => self.make_string()?,

            _ => {
                if self.is_digit(current_charecter) {
                    self.make_number();
                } else if self.is_alpha(current_charecter) {
                    self.make_identifier();
                } else {
                    return Err(Error::new(
                        self.line,
                        ErrorType::LexingError,
                        "Invalid charecter",
                    ));
                }
            }
        };

        Ok(())
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source_len
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_as_vec[self.current - 1] as char
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            return '\0';
        }

        return self.source_as_vec[self.current] as char;
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return self.source_as_vec[self.current + 1] as char;
    }

    fn does_match(&mut self, expected: char) -> bool {
        if self.is_eof() || self.peek() != expected {
            return false;
        };

        self.current += 1;

        return true;
    }

    fn is_digit(&self, charecter: char) -> bool {
        charecter >= '0' && charecter <= '9'
    }

    fn is_alpha(&self, charecter: char) -> bool {
        charecter >= 'a' && charecter <= 'z'
            || charecter >= 'A' && charecter <= 'Z'
            || charecter == '_'
    }

    fn is_alpha_numeric(&self, charecter: char) -> bool {
        self.is_alpha(charecter) || self.is_digit(charecter)
    }

    fn comment(&mut self) {
        while self.peek() != '\n' && !self.is_eof() {
            self.advance();
        }
    }

    fn multiline_comment(&mut self) -> Result<(), Error> {
        loop {
            if self.is_eof() {
                return Err(Error::new(
                    self.line,
                    ErrorType::LexingError,
                    "Unterminated comment",
                ));
            }

            match self.peek() {
                '*' => {
                    self.advance();
                    if self.does_match('/') {
                        return Ok(());
                    }
                }

                '/' => {
                    self.advance();
                    if self.does_match('*') {
                        self.multiline_comment()?;
                    }
                }

                '\n' => {
                    self.advance();
                    self.line += 1;
                }

                _ => {
                    self.advance();
                }
            }
        }
    }

    fn make_identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let identifier = self.source[self.start..self.current].to_string();
        if let Some(ttype) = self.keywords.get(&identifier) {
            self.add_token_without_literal(ttype.clone());
            return;
        };
        self.add_token_without_literal(TokenType::Identifier);
    }

    fn make_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance(); // consuming the dot
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let literal: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, Object::Number(literal));
    }

    fn make_string(&mut self) -> Result<(), Error> {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_eof() {
            return Err(Error::new(
                self.line,
                ErrorType::LexingError,
                "Unterminated String",
            ));
        }

        self.advance(); // covering up the ending qoute

        self.add_token(
            TokenType::String,
            Object::String(self.source[self.start..self.current].to_string()),
        );

        Ok(())
    }

    fn add_token_without_literal(&mut self, token_type: TokenType) {
        self.add_token(token_type, Object::Nil);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Object) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start..self.current].to_string(),
            literal,
            self.line,
        ));
    }

    fn init_keywords(&mut self) {
        self.keywords.insert(String::from("and"), TokenType::And);
        self.keywords
            .insert(String::from("class"), TokenType::Class);
        self.keywords.insert(String::from("else"), TokenType::Else);
        self.keywords
            .insert(String::from("false"), TokenType::False);
        self.keywords.insert(String::from("for"), TokenType::For);
        self.keywords.insert(String::from("fun"), TokenType::Fun);
        self.keywords.insert(String::from("nil"), TokenType::Nil);
        self.keywords.insert(String::from("if"), TokenType::If);
        self.keywords.insert(String::from("or"), TokenType::Or);
        self.keywords
            .insert(String::from("print"), TokenType::Print);
        self.keywords
            .insert(String::from("return"), TokenType::Return);
        self.keywords
            .insert(String::from("super"), TokenType::Super);
        self.keywords.insert(String::from("this"), TokenType::This);
        self.keywords.insert(String::from("true"), TokenType::True);
        self.keywords.insert(String::from("var"), TokenType::Var);
        self.keywords
            .insert(String::from("while"), TokenType::While);
    }
}
