use crate::{
    error::ReefError,
    expr::{Expr, Value},
    stmt::StmtKind,
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
    pub fn execute(&self, stmt: &StmtKind) -> Result<(), ReefError> {
        match stmt {
            StmtKind::Expression { expr } => Expr::evaluate(expr)?,
            StmtKind::Print { expr } => {
                let value = Expr::evaluate(expr)?;
                println!("{}", self.stringify(&value));
                value
            }
            StmtKind::Var { name: _, expr: _ } => {
                println!("var exectution not implemented yet");
                Value::Nil
            }
            _ => todo!(),
        };
        Ok(())
    }

    pub fn interpret(&self, stmts: Vec<StmtKind>) -> Result<(), ReefError> {
        for stmt in stmts {
            self.execute(&stmt)?
        }
        Ok(())
    }
}
impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}
