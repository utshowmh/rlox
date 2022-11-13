use crate::{
    error::Error,
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
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

    pub fn parse(&mut self) -> Result<Expr, Error> {
        self.exprs()
    }

    fn exprs(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.does_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        while self.does_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.does_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.does_match(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr::new(expr, operator, right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.does_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(UnaryExpr::new(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.does_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr::new(Object::True)));
        }

        if self.does_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr::new(Object::False)));
        }

        if self.does_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr::new(Object::Nil)));
        }

        if self.does_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr::new(self.previous().literal)));
        }

        if self.does_match(&[TokenType::LeftParen]) {
            let expr = self.exprs()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(GroupingExpr::new(expr)));
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
        Error::new(self.peek().line, &format!("ParseError -> {}", message))
    }
}
