use std::collections::HashMap;

use crate::{
    error::{Error, ErrorType},
    object::Object,
    token::Token,
};

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }

    pub fn define(&mut self, identifier: &str, value: Object) {
        self.values.insert(identifier.to_string(), value);
    }

    pub fn access(&self, identifier: &Token) -> Result<Object, Error> {
        if let Some(value) = self.values.get(&identifier.lexeme) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                identifier.line,
                ErrorType::RuntimeError,
                &format!("Undefined variable {}", identifier.lexeme),
            ))
        }
    }
}
