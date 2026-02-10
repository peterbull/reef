#![allow(unused_variables, dead_code)]

use std::{fmt, rc::Rc};

use crate::{Literal, Token, TokenType, error::ReefError, interpreter::Interpreter};

pub trait ReefCallable: fmt::Debug {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value, ReefError>;
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Rc<dyn ReefCallable>),
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

#[derive(Debug, Clone)]
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
    None,
}
