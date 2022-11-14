use crate::error::Error;
use crate::expression::{Expression, LiteralExpression};
use crate::object::Object;
use crate::token::Token;

pub enum Statement {
    VariableStatement(VariableStatement),
    ExpressionStatement(ExpressionStatement),
    PrintStatement(PrintStatement),
}

pub struct VariableStatement {
    pub identifier: Token,
    pub initializer: Expression,
}

pub struct ExpressionStatement {
    pub expression: Expression,
}

pub struct PrintStatement {
    pub expression: Expression,
}

impl ExpressionStatement {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

impl PrintStatement {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        match self {
            Self::VariableStatement(statement) => statement.accept(visitor),
            Self::ExpressionStatement(statement) => statement.accept(visitor),
            Self::PrintStatement(statement) => statement.accept(visitor),
        }
    }
}

pub trait StatementVisitor<T> {
    fn visit_expression_statement(&self, expression: &ExpressionStatement) -> Result<T, Error>;
    fn visit_print_statement(&self, expression: &PrintStatement) -> Result<T, Error>;
    fn visit_variable_statement(&mut self, expression: &VariableStatement) -> Result<T, Error>;
}

impl VariableStatement {
    pub fn new(identifier: Token, initializer: Option<Expression>) -> Self {
        Self {
            identifier,
            initializer: initializer
                .unwrap_or_else(|| Expression::Literal(LiteralExpression::new(Object::Nil))),
        }
    }

    fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_variable_statement(self)
    }
}

impl ExpressionStatement {
    fn accept<T>(&self, visitor: &dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_expression_statement(self)
    }
}

impl PrintStatement {
    fn accept<T>(&self, visitor: &dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_print_statement(self)
    }
}
