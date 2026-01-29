use std::collections::HashMap;

use crate::{Token, error::ReefError, expr::Value};

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }
    pub fn update_values(&mut self, name: String, value: Value) -> Result<Value, ReefError> {
        match self.values.insert(name, value) {
            Some(value) => Ok(value),
            None => Err(ReefError::reef_general_error(
                "error creating or updating variable value",
            )),
        }
    }

    pub fn define(&mut self, name: String, value: Value) -> Result<Value, ReefError> {
        self.update_values(name, value)
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<Value, ReefError> {
        if self.values.contains_key(&name) {
            self.update_values(name, value)
        } else {
            Err(ReefError::reef_general_error(&format!(
                "undefined variable: {:?}",
                name
            )))
        }
    }

    pub fn get(&self, name: &Token) -> Value {
        self.values
            .get(&name.lexeme)
            .expect("env should always be able to get this value")
            .clone()
    }
}
impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}
