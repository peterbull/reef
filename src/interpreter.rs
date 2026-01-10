use crate::{
    error::LoxError,
    expr::{Expr, ExprKind, Value},
};

pub struct Interpreter {}
impl Interpreter {
    pub fn eval_literal(expr: &ExprKind) -> Result<Value, LoxError> {
        Expr::evaluate(&expr)
    }
    pub fn eval_grouping(expr: &ExprKind) -> Result<Value, LoxError> {
        Expr::evaluate(&expr)
    }
}
