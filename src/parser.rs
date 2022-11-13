use crate::{
    error::{Error, ErrorType},
    expression::{
        BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression,
    },
    object::Object,
    token::Token,
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expression, Error> {
        self.expressions()
    }

    fn expressions(&mut self) -> Result<Expression, Error> {
        // let mut expression = self.equality()?;

        // while self.does_match(&[TokenType::Comma]) {
        //     expression = self.equality()?;
        // }

        // Ok(expression)

        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        let mut expression = self.comparison()?;

        while self.does_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expression = Expression::Binary(BinaryExpression::new(expression, operator, right));
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        let mut expression = self.term()?;

        while self.does_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expression = Expression::Binary(BinaryExpression::new(expression, operator, right));
        }

        return Ok(expression);
    }

    fn term(&mut self) -> Result<Expression, Error> {
        let mut expression = self.factor()?;

        while self.does_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expression = Expression::Binary(BinaryExpression::new(expression, operator, right));
        }

        return Ok(expression);
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        let mut expression = self.unary()?;

        while self.does_match(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expression = Expression::Binary(BinaryExpression::new(expression, operator, right));
        }

        return Ok(expression);
    }

    fn unary(&mut self) -> Result<Expression, Error> {
        if self.does_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expression::Unary(UnaryExpression::new(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        if self.does_match(&[TokenType::True]) {
            return Ok(Expression::Literal(LiteralExpression::new(Object::True)));
        }

        if self.does_match(&[TokenType::False]) {
            return Ok(Expression::Literal(LiteralExpression::new(Object::False)));
        }

        if self.does_match(&[TokenType::Nil]) {
            return Ok(Expression::Literal(LiteralExpression::new(Object::Nil)));
        }

        if self.does_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expression::Literal(LiteralExpression::new(
                self.previous().literal,
            )));
        }

        if self.does_match(&[TokenType::LeftParen]) {
            let expression = self.expressions()?;
            self.consume(TokenType::RightParen, "Expect ')' after expressionession")?;
            return Ok(Expression::Grouping(GroupingExpression::new(expression)));
        }

        Err(self.parse_error("Does not support this token as primary"))
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(&ttype) {
            return Ok(self.advance());
        }

        Err(self.parse_error(message))
    }

    fn does_match(&mut self, ttypes: &[TokenType]) -> bool {
        for ttype in ttypes {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().ttype == ttype
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    // fn synchronize(&mut self) {
    //     self.advance();

    //     while !self.is_at_end() {
    //         if (self.previous().ttype == TokenType::Semicolon) {
    //             return;
    //         };

    //         if matches!(
    //             self.peek().ttype,
    //             TokenType::Class
    //                 | TokenType::If
    //                 | TokenType::Fun
    //                 | TokenType::Var
    //                 | TokenType::For
    //                 | TokenType::While
    //                 | TokenType::Print
    //                 | TokenType::Return
    //         ) {
    //             return;
    //         }

    //         self.advance();
    //     }
    // }

    fn parse_error(&mut self, message: &str) -> Error {
        Error::new(self.peek().line, ErrorType::ParsingError, message)
    }
}
