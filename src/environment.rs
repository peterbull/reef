use std::collections::HashMap;

use crate::{Token, error::ReefError, expr::Value};

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Environment {
            enclosing: enclosing.map(Box::new),
            values: HashMap::new(),
        }
    }
    pub fn update_values(&mut self, name: String, value: Value) -> Result<Value, ReefError> {
        self.values.insert(name, value.clone());
        Ok(value)
    }

    pub fn define(&mut self, name: String, value: Value) -> Result<Value, ReefError> {
        self.update_values(name, value)
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<Value, ReefError> {
        if self.values.contains_key(&name.lexeme) {
            return self.update_values(name.lexeme.to_string(), value);
        }
        if let Some(ref mut enc) = self.enclosing {
            return enc.assign(name, value);
        }
        Err(ReefError::reef_general_error(&format!(
            "undefined variable: {:?}",
            name
        )))
    }

    pub fn get(&self, name: &Token) -> Result<Value, ReefError> {
        if let Some(val) = self.values.get(&name.lexeme) {
            return Ok(val.clone());
        }
        if let Some(enc) = &self.enclosing {
            return enc.get(name);
        }
        Err(ReefError::reef_general_error(&format!(
            "undefined variable: '{}'",
            name.lexeme
        )))
    }
}
