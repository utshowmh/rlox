use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

pub enum Expr {
    LiteralExpr(LiteralExpr),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
    GroupingExpr(GroupingExpr),
}

pub struct LiteralExpr {
    value: Object,
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExpr {
    exprs: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visitLiteralExpr(&self, expr: &LiteralExpr) -> Result<T, Error>;
    fn visitUnaryExpr(&self, expr: &UnaryExpr) -> Result<T, Error>;
    fn visitBinaryExpr(&self, expr: &BinaryExpr) -> Result<T, Error>;
    fn visitGroupingExpr(&self, expr: &GroupingExpr) -> Result<T, Error>;
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visitLiteralExpr(self)
    }
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visitUnaryExpr(self)
    }
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visitBinaryExpr(self)
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
        visitor.visitGroupingExpr(self)
    }
}
