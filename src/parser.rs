use crate::{
    error::{Error, ErrorType},
    expression::{
        BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression,
        VariableExpression,
    },
    object::Object,
    statement::{
        BlockStatement, ExpressionStatement, IfStatement, PrintStatement, Statement,
        VariableStatement,
    },
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

    pub fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Statement, Error> {
        if self.does_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        if self.does_match(&[TokenType::Print]) {
            self.print_statement()
        } else if self.does_match(&[TokenType::LeftBrace]) {
            self.block()
        } else if self.does_match(&[TokenType::If]) {
            self.if_statement()
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Statement, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'")?;
        let conditional = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition")?;
        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.does_match(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }
        Ok(Statement::IfStatement(IfStatement::new(
            conditional,
            then_branch,
            else_branch,
        )))
    }

    fn block(&mut self) -> Result<Statement, Error> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block")?;

        Ok(Statement::BlockStatement(BlockStatement::new(statements)))
    }

    fn var_declaration(&mut self) -> Result<Statement, Error> {
        let name = self.consume(TokenType::Identifier, "Expect identifier")?;
        let mut initializer = None;

        if self.does_match(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after declaration")?;

        Ok(Statement::VariableStatement(VariableStatement::new(
            name,
            initializer,
        )))
    }

    fn print_statement(&mut self) -> Result<Statement, Error> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after value")?;

        Ok(Statement::PrintStatement(PrintStatement::new(value)))
    }

    fn expression_statement(&mut self) -> Result<Statement, Error> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after value")?;

        Ok(Statement::ExpressionStatement(ExpressionStatement::new(
            value,
        )))
    }

    fn expression(&mut self) -> Result<Expression, Error> {
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

        if self.does_match(&[TokenType::Identifier]) {
            return Ok(Expression::VariableExpression(VariableExpression::new(
                self.previous(),
            )));
        }

        if self.does_match(&[TokenType::LeftParen]) {
            let expression = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expressionession")?;
            return Ok(Expression::Grouping(GroupingExpression::new(expression)));
        }

        Err(self.parse_error(&format!(
            "Does not support '{}' as a primary token",
            self.peek().ttype
        )))
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
