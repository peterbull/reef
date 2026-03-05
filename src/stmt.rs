use crate::{Token, environment::Environment, error::ReefError, expr::Expr};

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

pub struct Stmt {
    stmt: StmtKind,
}

impl Stmt {
    pub fn new(stmt: StmtKind) -> Self {
        Stmt { stmt }
    }
    pub fn execute(&mut self, stmt: &StmtKind) -> Result<(), ReefError> {
        match stmt {
            _ => Err(ReefError::reef_general_error(
                "statement type not implemented",
            )),
        }
    }
}
