use crate::{
    error::ReefError,
    expr::{Expr, ExprKind, Value},
    stmt::{Stmt, StmtKind},
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
            StmtKind::Expression { expr } => Expr::evaluate(&expr)?,
            StmtKind::Print { expr } => {
                let value = Expr::evaluate(&expr)?;
                println!("{}", self.stringify(&value));
                value
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
