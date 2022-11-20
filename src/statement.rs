use crate::error::Error;
use crate::expression::{Expression, LiteralExpression};
use crate::object::Object;
use crate::token::Token;

pub enum Statement {
    VariableStatement(VariableStatement),
    ExpressionStatement(ExpressionStatement),
    PrintStatement(PrintStatement),
    BlockStatement(BlockStatement),
    IfStatement(IfStatement),
}

pub struct VariableStatement {
    pub identifier: Token,
    pub initializer: Expression,
}

pub struct ExpressionStatement {
    pub expression: Expression,
}

pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

pub struct IfStatement {
    pub conditional: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
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
            Self::BlockStatement(statement) => statement.accept(visitor),
            Self::IfStatement(statement) => statement.accept(visitor),
        }
    }
}

pub trait StatementVisitor<T> {
    fn visit_expression_statement(&self, statement: &ExpressionStatement) -> Result<T, Error>;
    fn visit_print_statement(&self, statement: &PrintStatement) -> Result<T, Error>;
    fn visit_variable_statement(&mut self, statement: &VariableStatement) -> Result<T, Error>;
    fn visit_block_statement(&mut self, statement: &BlockStatement) -> Result<T, Error>;
    fn visit_if_statement(&mut self, statement: &IfStatement) -> Result<T, Error>;
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

impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_block_statement(self)
    }
}

impl IfStatement {
    pub fn new(
        conditional: Expression,
        then_branch: Statement,
        else_branch: Option<Statement>,
    ) -> Self {
        Self {
            conditional,
            then_branch: Box::new(then_branch),
            else_branch: if let Some(else_branch) = else_branch {
                Some(Box::new(else_branch))
            } else {
                None
            },
        }
    }

    fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_if_statement(self)
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
