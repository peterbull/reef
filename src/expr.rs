#![allow(unused_variables, dead_code)]

use crate::{Literal, Token, TokenType, error::ReefError};

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    // funcs etc to be added
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

#[derive(Debug)]
pub struct Expr {}
impl Expr {
    // TODO: move this stuff to a module or a submodule
    fn check_number_operand(operator: &Token, right_operand: &Value) -> Result<(), ReefError> {
        match right_operand {
            Value::Number(_) => Ok(()),
            _ => Err(ReefError::reef_runtime_error(
                operator,
                "operand must be a number",
            )),
        }
    }
    fn check_number_operands(
        operator: &Token,
        left_operand: &Value,
        right_operand: &Value,
    ) -> Result<(), ReefError> {
        match (left_operand, right_operand) {
            (Value::Number(_), Value::Number(_)) => Ok(()),
            _ => Err(ReefError::reef_runtime_error(
                operator,
                "operands must be a number",
            )),
        }
    }
    fn is_equal(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            (Value::Nil, Value::Nil) => true,
            (_, Value::Nil) => false,
            (Value::Nil, _) => false,
            _ => false,
        }
    }

    fn evaluate_binary(
        left: &ExprKind,
        operator: &Token,
        right: &ExprKind,
    ) -> Result<Value, ReefError> {
        let left_val = Expr::evaluate(left)?;
        let right_val = Expr::evaluate(right)?;
        match operator.token_type {
            TokenType::Plus => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => {
                    let addition_result = l + r;
                    Ok(Value::Number(addition_result))
                }
                (Value::String(l), Value::String(r)) => {
                    let concat_result = format!("{}{}", l, r);
                    Ok(Value::String(concat_result))
                }

                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::Minus => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => {
                    let subtraction_result = l - r;
                    Ok(Value::Number(subtraction_result))
                }
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::Star => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => {
                    let multiplication_result = l * r;
                    Ok(Value::Number(multiplication_result))
                }
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::Slash => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => {
                    let division_result = l / r;
                    Ok(Value::Number(division_result))
                }
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },

            TokenType::EqualEqual => Ok(Value::Boolean(Expr::is_equal(&left_val, &right_val))),
            TokenType::BangEqual => Ok(Value::Boolean(!Expr::is_equal(&left_val, &right_val))),
            TokenType::GreaterEqual => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::Greater => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::LessEqual => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::Less => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            _ => Err(ReefError::reef_runtime_error(
                operator,
                "Binary evaluation error",
            )),
        }
    }

    fn evaluate_unary(operator: &Token, right: &ExprKind) -> Result<Value, ReefError> {
        let right_val = Expr::evaluate(right)?;
        match operator.token_type {
            TokenType::Minus => match right_val {
                Value::Number(right_val) => Ok(Value::Number(-right_val)),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Operand must be a number",
                )),
            },
            TokenType::Bang => Ok(Value::Boolean(!right_val.is_truthy())),
            _ => Err(ReefError::reef_runtime_error(
                operator,
                "invalid unary operator",
            )),
        }
    }

    fn evaluate_literal(value: &Literal) -> Result<Value, ReefError> {
        Ok(match value {
            Literal::String(s) => Value::String(s.clone()),
            Literal::Number(n) => Value::Number(*n),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Nil => Value::Nil,
        })
    }

    pub fn evaluate(expr: &ExprKind) -> Result<Value, ReefError> {
        match expr {
            // ExprKind::Assign { name, value } => {}
            ExprKind::Binary {
                left,
                operator,
                right,
            } => Expr::evaluate_binary(left, operator, right),
            // ExprKind::Call {
            //     callee,
            //     token,
            //     arguments,
            // } => {}
            // ExprKind::Get { object, name } => {}
            ExprKind::Grouping { expression } => Expr::evaluate(expression),
            ExprKind::Literal { value } => Expr::evaluate_literal(value),
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
            ExprKind::Unary { operator, right } => Expr::evaluate_unary(operator, right),
            // ExprKind::Variable { name } => {}
            _ => todo!(),
        }
    }
}
