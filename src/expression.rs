use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

pub enum Expression {
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
}

impl Expression {
    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        match self {
            Self::Literal(expression) => expression.accept(visitor),
            Self::Unary(expression) => expression.accept(visitor),
            Self::Binary(expression) => expression.accept(visitor),
            Self::Grouping(expression) => expression.accept(visitor),
        }
    }
}

pub struct LiteralExpression {
    pub value: Object,
}

pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

pub struct GroupingExpression {
    pub expressions: Box<Expression>,
}

pub trait ExpressionVisitor<T> {
    fn visit_literal_expression(&self, expression: &LiteralExpression) -> Result<T, Error>;
    fn visit_unary_expression(&self, expression: &UnaryExpression) -> Result<T, Error>;
    fn visit_binary_expression(&self, expression: &BinaryExpression) -> Result<T, Error>;
    fn visit_grouping_expression(&self, expression: &GroupingExpression) -> Result<T, Error>;
}

impl LiteralExpression {
    pub fn new(value: Object) -> Self {
        Self { value }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_literal_expression(self)
    }
}

impl UnaryExpression {
    pub fn new(operator: Token, right: Expression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_unary_expression(self)
    }
}

impl BinaryExpression {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_binary_expression(self)
    }
}

impl GroupingExpression {
    pub fn new(expressions: Expression) -> Self {
        Self {
            expressions: Box::new(expressions),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_grouping_expression(self)
    }
}
