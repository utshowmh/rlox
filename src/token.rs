use core::fmt;

use crate::{object::Object, token_type::TokenType};

#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token: '{}' of type '{}' (Object: {}) in line {}",
            self.lexeme, self.ttype, self.literal, self.line
        )
    }
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

    // pub fn to_string(&self) -> String {
    //     format!(
    //         "Token: '{}' (object: {:?}) of type '{:?}' in line {}",
    //         self.lexeme, self.literal, self.ttype, self.line
    //     )
    // }
}
