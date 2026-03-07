#![allow(unused_variables, dead_code)]

use crate::class::{ReefClassRef, ReefInstance, ReefInstanceRef};
use crate::{
    Literal, Token, TokenType, error::ReefError, func::ReefCallable, interpreter::Interpreter,
};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Rc<dyn ReefCallable>),
    Instance(ReefInstanceRef),
}

impl Value {
    pub fn as_number(&self) -> Result<f64, ReefError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(ReefError::reef_general_error(&format!(
                "Expected number, got {:?}",
                self
            ))),
        }
    }
    pub fn as_string(&self) -> Result<&str, ReefError> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(ReefError::reef_general_error(&format!(
                "Expected string, got {:?}",
                self
            ))),
        }
    }
    pub fn as_boolean(&self) -> Result<bool, ReefError> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(ReefError::reef_general_error(&format!(
                "Expected boolean, got {:?}",
                self
            ))),
        }
    }
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }
}

pub type Expr = Rc<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Assign {
        name: Token,
        value: Expr,
    },
    Binary {
        left: Expr,
        operator: Token,
        right: Expr,
    },
    Call {
        callee: Expr,
        token: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Expr,
        name: Token,
    },
    Grouping {
        expression: Expr,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Expr,
        operator: Token,
        right: Expr,
    },
    Set {
        object: Expr,
        name: Token,
        value: Expr,
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
        right: Expr,
    },
    Variable {
        name: Token,
    },
    None,
}
