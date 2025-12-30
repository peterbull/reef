#![allow(unused_variables, dead_code)]
use crate::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        token: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
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
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

pub fn evaluate(expr: &Expr) {
    match expr {
        Expr::Assign { name, value } => {}
        Expr::Binary {
            left,
            operator,
            right,
        } => {}
        Expr::Call {
            callee,
            token,
            arguments,
        } => {}
        Expr::Get { object, name } => {}
        Expr::Grouping { expression } => {}
        Expr::Literal { value } => {}
        Expr::Logical {
            left,
            operator,
            right,
        } => {}
        Expr::Set {
            object,
            name,
            value,
        } => {}
        Expr::Super { keyword, method } => {}
        Expr::This { keyword } => {}
        Expr::Unary { operator, right } => {}
        Expr::Variable { name } => {}
    }
}
