use crate::{
    error::ReefError,
    expr::{Expr, ExprKind, Value},
};

pub struct Interpreter {}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }
    pub fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => n.to_string(),
            Value::Boolean(n) => n.to_string(),
            Value::String(n) => n.to_string(),
            Value::Nil => String::from("nil"),
        }
    }
    pub fn interpret(&self, expr_kind: &ExprKind) -> Result<Value, ReefError> {
        let expr = Expr::new();
        let value = expr.evaluate(expr_kind)?;
        println!("result: {}", self.stringify(&value));
        Ok(value)
    }
}
