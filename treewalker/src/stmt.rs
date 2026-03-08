use crate::{Token, error::ReefError, expr::Expr};

#[derive(Debug, Clone)]
pub enum StmtKind {
    Print {
        expr: Expr,
    },
    Expression {
        expr: Expr,
    },
    Var {
        name: Token,
        initializer: Expr,
    },
    Return {
        keyword: Token,
        value: Expr,
    },
    Block {
        statements: Vec<StmtKind>,
    },
    If {
        condition: Expr,
        then_branch: Box<StmtKind>,
        else_branch: Option<Box<StmtKind>>,
    },
    Error {
        e: ReefError,
    },
    While {
        condition: Expr,
        body: Box<StmtKind>,
    },
    Function {
        name: Token,
        parameters: Vec<Token>,
        body: Vec<StmtKind>,
    },
    Class {
        name: Token,
        methods: Vec<StmtKind>,
    },
}
