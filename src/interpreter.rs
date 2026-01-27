use crate::{
    environment::{self, Environment},
    error::ReefError,
    expr::{Expr, ExprKind, Value},
    stmt::StmtKind,
};

pub struct Interpreter {
    environment: Environment,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }
    pub fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => n.to_string(),
            Value::Boolean(n) => n.to_string(),
            Value::String(n) => n.to_string(),
            Value::Nil => String::from("nil"),
        }
    }
    pub fn execute(&mut self, stmt: &StmtKind) -> Result<(), ReefError> {
        match stmt {
            StmtKind::Expression { expr } => Expr::evaluate(expr)?,
            StmtKind::Print { expr } => {
                let value = Expr::evaluate(expr)?;
                println!("{}", self.stringify(&value));
                value
            }
            StmtKind::Var {
                name: _,
                initializer,
            } => {
                todo!()
                // let mut value = Value::Nil;
                // match initializer {
                //     ExprKind::None => {}
                //     _ => {}
                // }
            }
            _ => todo!(),
        };
        Ok(())
    }

    pub fn interpret(&mut self, stmts: Vec<StmtKind>) -> Result<(), ReefError> {
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
