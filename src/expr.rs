use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

pub enum Expr {
    LiteralExpr(LiteralExpr),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
    GroupingExpr(GroupingExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        match self {
            Self::LiteralExpr(expr) => expr.accept(visitor),
            Self::UnaryExpr(expr) => expr.accept(visitor),
            Self::BinaryExpr(expr) => expr.accept(visitor),
            Self::GroupingExpr(expr) => expr.accept(visitor),
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
    fn visitLiteralExpr(&self, expr: &LiteralExpr) -> Result<T, Error>;
    fn visitUnaryExpr(&self, expr: &UnaryExpr) -> Result<T, Error>;
    fn visitBinaryExpr(&self, expr: &BinaryExpr) -> Result<T, Error>;
    fn visitGroupingExpr(&self, expr: &GroupingExpr) -> Result<T, Error>;
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visitLiteralExpr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visitUnaryExpr(self)
    }
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visitBinaryExpr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visitGroupingExpr(self)
    }
}
