use crate::error::Error;
use crate::expression::Expression;

pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    PrintStatement(PrintStatement),
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
    pub fn accept<T>(&self, visitor: &dyn StatementVisitor<T>) -> Result<T, Error> {
        match self {
            Statement::ExpressionStatement(statement) => statement.accept(visitor),
            Statement::PrintStatement(statement) => statement.accept(visitor),
        }
    }
}

pub trait StatementVisitor<T> {
    fn visit_expression_statement(&self, expression: &ExpressionStatement) -> Result<T, Error>;
    fn visit_print_statement(&self, expression: &PrintStatement) -> Result<T, Error>;
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
