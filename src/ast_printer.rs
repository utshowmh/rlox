use crate::{
    error::Error,
    expression::{
        BinaryExpression, Expression, ExpressionVisitor, GroupingExpression, LiteralExpression,
        UnaryExpression,
    },
};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn stringify(&self, expression: &Expression) -> Result<String, Error> {
        expression.accept(self)
    }

    fn parenthesize(
        &self,
        operator_lexeme: &str,
        expressions: &[&Box<Expression>],
    ) -> Result<String, Error> {
        let mut builder = format!("({}", operator_lexeme);

        for expression in expressions {
            builder = format!("{} {}", builder, expression.accept(self)?);
        }

        builder = format!("{})", builder);

        Ok(builder)
    }
}

impl ExpressionVisitor<String> for AstPrinter {
    fn visit_literal_expression(&self, expression: &LiteralExpression) -> Result<String, Error> {
        Ok(format!("{}", expression.value))
    }

    fn visit_unary_expression(&self, expression: &UnaryExpression) -> Result<String, Error> {
        self.parenthesize(&expression.operator.lexeme, &[&expression.right])
    }

    fn visit_binary_expression(&self, expression: &BinaryExpression) -> Result<String, Error> {
        self.parenthesize(
            &expression.operator.lexeme,
            &[&expression.left, &expression.right],
        )
    }

    fn visit_grouping_expression(&self, expression: &GroupingExpression) -> Result<String, Error> {
        self.parenthesize("group", &[&expression.expressions])
    }
}
