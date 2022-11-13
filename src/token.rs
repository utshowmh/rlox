use crate::{object::Object, token_type::TokenType};

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
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
