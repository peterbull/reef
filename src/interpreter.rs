use crate::{
    error::LoxError,
    expr::{Expr, ExprKind, Value},
};

pub struct Interpreter {}
impl Interpreter {
    pub fn interpret(expr_kind: &ExprKind) -> Result<Value, LoxError> {
        let expr = Expr::new();
        let value = expr.evaluate(expr_kind);
        println!("{:?}", &value);
        value
    }
}
