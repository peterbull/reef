use std::collections::HashMap;

use crate::{Token, error::ReefError, expr::ExprKind};

pub struct Environment {
    values: HashMap<String, ExprKind>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: ExprKind) -> Option<ExprKind> {
        self.values.insert(name, value)
    }
    pub fn get(&self, name: &Token) -> Result<&ExprKind, ReefError> {
        if let Some(name) = self.values.get(&name.lexeme) {
            return Ok(name);
        };
        Err(ReefError::reef_runtime_error(
            name,
            &format!("Undefined variable: {}.", name.lexeme),
        ))
    }
}
impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}
