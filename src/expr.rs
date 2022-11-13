use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

pub enum Expr {
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        match self {
            Self::Literal(expr) => expr.accept(visitor),
            Self::Unary(expr) => expr.accept(visitor),
            Self::Binary(expr) => expr.accept(visitor),
            Self::Grouping(expr) => expr.accept(visitor),
        }
    }
}

pub struct LiteralExpr {
    pub value: Object,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub exprs: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, Error>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, Error>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, Error>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, Error>;
}

impl LiteralExpr {
    pub fn new(value: Object) -> Self {
        Self { value }
    }

    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visit_unary_expr(self)
    }
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn new(exprs: Expr) -> Self {
        Self {
            exprs: Box::new(exprs),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visit_grouping_expr(self)
    }
}
