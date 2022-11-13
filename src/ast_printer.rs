use std::vec;

use crate::{
    error::Error,
    expr::{BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr},
};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn stringify(&self, expr: &Expr) -> Result<String, Error> {
        expr.accept(self)
    }

    fn parenthesize(&self, operator_lexeme: &str, exprs: &[&Box<Expr>]) -> Result<String, Error> {
        let mut builder = format!("({})", operator_lexeme);

        for expr in exprs {
            builder = format!("{} {:?}", builder, expr.accept(self)?);
        }

        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visitLiteralExpr(&self, expr: &LiteralExpr) -> Result<String, Error> {
        Ok(format!("{:?}", expr.value))
    }

    fn visitUnaryExpr(&self, expr: &UnaryExpr) -> Result<String, Error> {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.right])
    }

    fn visitBinaryExpr(&self, expr: &BinaryExpr) -> Result<String, Error> {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.left, &expr.right])
    }

    fn visitGroupingExpr(&self, expr: &GroupingExpr) -> Result<String, Error> {
        self.parenthesize("group", &vec![&expr.exprs])
    }
}
