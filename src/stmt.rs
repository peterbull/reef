use crate::{error::ReefError, expr::ExprKind};

#[derive(Debug)]
pub enum StmtKind {
    Print { expression: Box<ExprKind> },
    Expression { expression: Box<ExprKind> },
}
pub struct Stmt {}
impl Stmt {
    pub fn execute(&mut self, stmt: &StmtKind) -> Result<(), ReefError> {
        match stmt {
            _ => Err(ReefError::reef_general_error(
                "statement type not implemented",
            )),
        }
    }
}
