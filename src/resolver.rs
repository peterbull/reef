#![allow(unused_variables, dead_code)]
use std::collections::HashMap;

use crate::{
    ExprKind, Token,
    error::ReefError,
    interpreter::{self, Interpreter},
    stmt::StmtKind,
};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        let scopes: Vec<HashMap<String, bool>> = Vec::new();
        Resolver {
            interpreter,
            scopes,
        }
    }

    pub fn resolve(&mut self, statements: Vec<StmtKind>) -> Result<(), ReefError> {
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, statement: StmtKind) -> Result<(), ReefError> {
        match statement {
            StmtKind::Block { statements } => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
                Ok(())
            }
            StmtKind::Var { name, initializer } => {
                self.resolve_var_decl(&name, initializer).unwrap();
                Ok(())
            }
            StmtKind::Function {
                name,
                parameters,
                body,
            } => {
                self.declare(&name);
                self.define(&name);
                self.resolve_fn(parameters, body)?;
                Ok(())
            }
            StmtKind::Expression { expr } => {
                self.resolve_expr(expr)?;
                Ok(())
            }
            _ => todo!("finish statement resolutions"),
        }
    }

    fn resolve_expr(&mut self, expression: ExprKind) -> Result<(), ReefError> {
        match expression.clone() {
            ExprKind::Variable { name } => self.resolve_var_expr(&name, expression),
            ExprKind::Assign { name, value } => self.resolve_assignment(&name, *value, expression),
            _ => todo!("finish expression resolutions"),
        }
    }

    fn resolve_var_decl(&mut self, name: &Token, initializer: ExprKind) -> Result<(), ReefError> {
        self.declare(name);
        match initializer {
            ExprKind::None => {}
            _ => self.resolve_expr(initializer)?,
        }
        self.define(name);
        Ok(())
    }

    fn resolve_var_expr(&mut self, name: &Token, expression: ExprKind) -> Result<(), ReefError> {
        if !self.scopes.is_empty() && self.scopes.last().unwrap().get(&name.lexeme) == Some(&false)
        {
            return Err(ReefError::reef_general_error(
                "Can't read local variable in its own initializer.",
            ));
        }
        self.resolve_local(expression, name)?;
        Ok(())
    }

    fn resolve_assignment(
        &mut self,
        name: &Token,
        value: ExprKind,
        expression: ExprKind,
    ) -> Result<(), ReefError> {
        self.resolve_expr(value)?;
        self.resolve_local(expression, name)?;
        Ok(())
    }

    fn resolve_fn(&mut self, parameters: Vec<Token>, body: Vec<StmtKind>) -> Result<(), ReefError> {
        self.begin_scope();
        for param in parameters {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve(body)?;
        self.end_scope();
        Ok(())
    }

    fn resolve_local(&mut self, expr: ExprKind, name: &Token) -> Result<(), ReefError> {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(expr.clone(), self.scopes.len() - 1 - i);
            }
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .expect("expect scope to exist")
            .insert(name.lexeme.to_string(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .expect("expect scope to exist")
            .insert(name.lexeme.to_string(), true);
    }
}
