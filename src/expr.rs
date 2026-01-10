#![allow(unused_variables, dead_code)]
use crate::{Literal, Token, error::LoxError};

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    // funcs etc to be added
}

#[derive(Debug)]
pub enum ExprKind {
    Assign {
        name: Token,
        value: Box<ExprKind>,
    },
    Binary {
        left: Box<ExprKind>,
        operator: Token,
        right: Box<ExprKind>,
    },
    Call {
        callee: Box<ExprKind>,
        token: Token,
        arguments: Vec<ExprKind>,
    },
    Get {
        object: Box<ExprKind>,
        name: Token,
    },
    Grouping {
        expression: Box<ExprKind>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<ExprKind>,
        operator: Token,
        right: Box<ExprKind>,
    },
    Set {
        object: Box<ExprKind>,
        name: Token,
        value: Box<ExprKind>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<ExprKind>,
    },
    Variable {
        name: Token,
    },
}

#[derive(Debug)]
pub struct Expr {}
impl Expr {
    pub fn evaluate(data: &ExprKind) -> Result<Value, LoxError> {
        match data {
            // ExprKind::Assign { name, value } => {}
            // ExprKind::Binary {
            //     left,
            //     operator,
            //     right,
            // } => {}
            // ExprKind::Call {
            //     callee,
            //     token,
            //     arguments,
            // } => {}
            // ExprKind::Get { object, name } => {}
            // ExprKind::Grouping { expression } => {}
            ExprKind::Literal { value } => Ok(match value {
                Literal::String(s) => Value::String(s.clone()),
                Literal::Number(n) => Value::Number(n.clone()),
                Literal::Boolean(b) => Value::Boolean(b.clone()),
                Literal::Nil => Value::Nil,
            }),
            // ExprKind::Logical {
            //     left,
            //     operator,
            //     right,
            // } => {}
            // ExprKind::Set {
            //     object,
            //     name,
            //     value,
            // } => {}
            // ExprKind::Super { keyword, method } => {}
            // ExprKind::This { keyword } => {}
            // ExprKind::Unary { operator, right } => {}
            // ExprKind::Variable { name } => {}
            _ => Ok(Value::Nil), // placeholder to keep rust happy while i fill out the others
        }
    }
}
