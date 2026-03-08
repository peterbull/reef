use crate::{Literal, expr::ExprKind};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expr: &ExprKind) -> String {
        match expr {
            ExprKind::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    Self::print(left),
                    Self::print(right)
                )
            }
            ExprKind::Grouping { expression } => {
                format!("(group {})", Self::print(expression))
            }
            ExprKind::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, Self::print(right))
            }
            ExprKind::Literal { value } => Self::print_literal(value),
            _ => String::from("expression not implemented yet"),
        }
    }
    pub fn print_literal(literal: &Literal) -> String {
        match literal {
            Literal::Number(n) => n.to_string(),
            Literal::Boolean(n) => n.to_string(),
            Literal::String(n) => n.to_string(),
            Literal::Nil => String::from("nil"),
        }
    }
}
