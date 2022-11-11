use crate::{object::Object, token_type::TokenType};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Object,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
        Self {
            ttype: token_type,
            lexeme,
            literal,
            line,
        }
    }
}
