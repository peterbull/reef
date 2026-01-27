use crate::{Token, error::ReefError, expr::ExprKind};

#[derive(Debug, Clone)]
pub enum StmtKind {
    Print { expr: ExprKind },
    Expression { expr: ExprKind },
    Var { name: Token, initializer: ExprKind },
    Error { e: ReefError },
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
