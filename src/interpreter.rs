use crate::{
    environment::Environment,
    error::{Error, ErrorType},
    expression::{Expression, ExpressionVisitor, LiteralExpression},
    object::Object,
    statement::{
        BlockStatement, ExpressionStatement, IfStatement, PrintStatement, Statement,
        StatementVisitor, VariableStatement,
    },
    token_type::TokenType,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Statement]) -> Result<(), Error> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Statement) -> Result<Object, Error> {
        statement.accept(self)
    }

    fn execute_block(&mut self, statements: &Vec<Statement>) -> Result<Object, Error> {
        let previous = self.environment.clone();

        for statement in statements {
            self.execute(statement)?;
        }
        self.environment = previous;

        Ok(Object::Nil)
    }

    fn evaluate(&self, expression: &Expression) -> Result<Object, Error> {
        expression.accept(self)
    }

    fn is_truthy(&self, object: Object) -> bool {
        !matches!(object, Object::Nil | Object::False)
    }
}

impl StatementVisitor<Object> for Interpreter {
    fn visit_expression_statement(&self, statement: &ExpressionStatement) -> Result<Object, Error> {
        let value = self.evaluate(&statement.expression)?;
        Ok(value)
    }

    fn visit_print_statement(&self, statement: &PrintStatement) -> Result<Object, Error> {
        let value = self.evaluate(&statement.expression)?;
        println!("{}", value);
        Ok(value)
    }

    fn visit_variable_statement(
        &mut self,
        expression: &VariableStatement,
    ) -> Result<Object, Error> {
        let value = self.evaluate(&expression.initializer)?;
        self.environment
            .define(&expression.identifier.lexeme, value.clone());
        Ok(value)
    }

    fn visit_block_statement(&mut self, statement: &BlockStatement) -> Result<Object, Error> {
        self.execute_block(&statement.statements)
    }

    fn visit_if_statement(&mut self, statement: &IfStatement) -> Result<Object, Error> {
        if self.is_truthy(self.evaluate(&statement.conditional)?) {
            self.execute(&statement.then_branch)
        } else {
            if let Some(else_branch) = &statement.else_branch {
                self.execute(else_branch)
            } else {
                Ok(Object::Nil)
            }
        }
    }
}

impl ExpressionVisitor<Object> for Interpreter {
    fn visit_literal_expression(&self, expression: &LiteralExpression) -> Result<Object, Error> {
        Ok(expression.value.clone())
    }

    fn visit_unary_expression(
        &self,
        expression: &crate::expression::UnaryExpression,
    ) -> Result<Object, Error> {
        let right = self.evaluate(&expression.right)?;

        let operator = &expression.operator;

        match operator.ttype {
            TokenType::Minus => match right {
                Object::Number(num) => Ok(Object::Number(-num)),
                _ => Ok(Object::Nil),
            },

            TokenType::Bang => {
                if self.is_truthy(right) {
                    Ok(Object::False)
                } else {
                    Ok(Object::True)
                }
            }

            _ => Err(Error::new(
                operator.line,
                ErrorType::RuntimeError,
                "Operator does not support unary operation",
            )),
        }
    }

    fn visit_binary_expression(
        &self,
        expression: &crate::expression::BinaryExpression,
    ) -> Result<Object, Error> {
        let left = self.evaluate(&expression.left)?;
        let right = self.evaluate(&expression.right)?;
        let operator = &expression.operator;

        match operator.ttype {
            TokenType::Plus => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left + right)),
                (Object::String(left), Object::String(right)) => {
                    Ok(Object::String(format!("{}{}", left, right)))
                }
                (_, _) => Err(Error::new(
                    operator.line,
                    ErrorType::RuntimeError,
                    "Operand must be either number or string",
                )),
            },

            TokenType::Minus => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left - right)),
                (_, _) => Err(Error::new(
                    operator.line,
                    ErrorType::RuntimeError,
                    "Operand must be a number",
                )),
            },

            TokenType::Star => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left * right)),
                (_, _) => Err(Error::new(
                    operator.line,
                    ErrorType::RuntimeError,
                    "Operand must be a number",
                )),
            },

            TokenType::Slash => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left / right)),
                (_, _) => Err(Error::new(
                    operator.line,
                    ErrorType::RuntimeError,
                    "Operand must be a number",
                )),
            },

            TokenType::EqualEqual => {
                if left == right {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }

            TokenType::BangEqual => {
                if left != right {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }

            TokenType::Greater => match (left, right) {
                (Object::Number(left), Object::Number(right)) => {
                    if left > right {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (Object::String(left), Object::String(right)) => {
                    if left > right {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (_, _) => Err(Error::new(
                    operator.line,
                    ErrorType::RuntimeError,
                    "Operand must be either number or string",
                )),
            },

            TokenType::GreaterEqual => match (left, right) {
                (Object::Number(left), Object::Number(right)) => {
                    if left >= right {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (Object::String(left), Object::String(right)) => {
                    if left >= right {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (_, _) => Err(Error::new(
                    operator.line,
                    ErrorType::RuntimeError,
                    "Operand must be either number or string",
                )),
            },

            TokenType::Less => match (left, right) {
                (Object::Number(left), Object::Number(right)) => {
                    if left < right {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (Object::String(left), Object::String(right)) => {
                    if left < right {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (_, _) => Err(Error::new(
                    operator.line,
                    ErrorType::RuntimeError,
                    "Operand must be either number or string",
                )),
            },

            TokenType::LessEqual => match (left, right) {
                (Object::Number(left), Object::Number(right)) => {
                    if left <= right {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (Object::String(left), Object::String(right)) => {
                    if left < right {
                        Ok(Object::True)
                    } else {
                        Ok(Object::False)
                    }
                }
                (_, _) => Err(Error::new(
                    operator.line,
                    ErrorType::RuntimeError,
                    "Operand must be either number or string",
                )),
            },

            _ => Err(Error::new(
                operator.line,
                ErrorType::RuntimeError,
                "Operator does not support binary opertaion",
            )),
        }
    }

    fn visit_grouping_expression(
        &self,
        expression: &crate::expression::GroupingExpression,
    ) -> Result<Object, Error> {
        Ok(self.evaluate(&expression.expressions)?)
    }

    fn visit_variable_expression(
        &self,
        expression: &crate::expression::VariableExpression,
    ) -> Result<Object, Error> {
        self.environment.access(&expression.identifier)
    }
}
