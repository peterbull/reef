use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    Token,
    error::ReefError,
    expr::{Expr, ExprKind},
    func::FunctionKind,
    interpreter::Interpreter,
    stmt::StmtKind,
};

pub struct Resolver {
    pub interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionKind,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        let scopes: Vec<HashMap<String, bool>> = Vec::new();
        Resolver {
            interpreter,
            scopes,
            current_function: FunctionKind::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<StmtKind>) -> Result<(), ReefError> {
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, statement: &StmtKind) -> Result<(), ReefError> {
        match statement {
            StmtKind::Block { statements } => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
                Ok(())
            }
            StmtKind::Var { name, initializer } => {
                self.resolve_var_decl(name, initializer)?;
                Ok(())
            }
            StmtKind::Function {
                name,
                parameters,
                body,
            } => {
                self.declare(name)?;
                self.define(name);
                self.resolve_fn(parameters, body, FunctionKind::Function)?;
                Ok(())
            }
            StmtKind::Expression { expr } => {
                self.resolve_expr(expr)?;
                Ok(())
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(branch) = else_branch {
                    self.resolve_stmt(branch)?;
                };
                Ok(())
            }
            StmtKind::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
                Ok(())
            }
            StmtKind::Print { expr } => {
                self.resolve_expr(expr)?;
                Ok(())
            }
            StmtKind::Return { keyword: _, value } => self.resolve_return(value),
            StmtKind::Class { name, methods: _ } => {
                self.declare(name)?;
                self.define(name);
                Ok(())
            }
            _ => todo!("finish statement resolutions"),
        }
    }

    fn resolve_return(&mut self, value: &Expr) -> Result<(), ReefError> {
        match value.as_ref() {
            ExprKind::None => Ok(()),
            _ => match self.current_function {
                FunctionKind::None => {
                    Err(ReefError::reef_general_error("No top level return allowed"))
                }
                _ => {
                    self.resolve_expr(value)?;
                    Ok(())
                }
            },
        }
    }

    fn resolve_expr(&mut self, expression: &Expr) -> Result<(), ReefError> {
        match expression.as_ref() {
            ExprKind::Variable { name } => self.resolve_var_expr(name, expression),
            ExprKind::Assign { name, value } => self.resolve_assignment(name, value, expression),
            ExprKind::Binary {
                left,
                operator: _operator,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            ExprKind::Call {
                callee,
                token: _token,
                arguments,
            } => {
                self.resolve_expr(callee)?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }
                Ok(())
            }
            ExprKind::Grouping { expression } => {
                self.resolve_expr(expression)?;
                Ok(())
            }
            ExprKind::Logical {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            ExprKind::Literal { value: _ } => Ok(()),
            ExprKind::Unary {
                operator: _operator,
                right,
            } => {
                self.resolve_expr(right)?;
                Ok(())
            }
            ExprKind::None => Ok(()),
            _ => todo!("finish expression resolutions"),
        }
    }

    fn resolve_var_decl(&mut self, name: &Token, initializer: &Expr) -> Result<(), ReefError> {
        self.declare(name)?;
        match initializer.as_ref() {
            ExprKind::None => {}
            _ => self.resolve_expr(initializer)?,
        }
        self.define(name);
        Ok(())
    }

    fn resolve_var_expr(&mut self, name: &Token, expression: &Expr) -> Result<(), ReefError> {
        if !self.scopes.is_empty() {
            let top_of_stack = self
                .scopes
                .last()
                .expect("expect top of stack to exist")
                .get(&name.lexeme);

            if top_of_stack == Some(&false) {
                return Err(ReefError::reef_general_error(
                    "Can't read local variable in its own initializer.",
                ));
            }
        }
        self.resolve_local(expression, name)?;
        Ok(())
    }

    fn resolve_assignment(
        &mut self,
        name: &Token,
        value: &Expr,
        expression: &Expr,
    ) -> Result<(), ReefError> {
        self.resolve_expr(value)?;
        self.resolve_local(expression, name)?;
        Ok(())
    }

    fn resolve_fn(
        &mut self,
        parameters: &Vec<Token>,
        body: &Vec<StmtKind>,
        fn_type: FunctionKind,
    ) -> Result<(), ReefError> {
        let enclosing_fn = self.current_function.clone();
        self.current_function = fn_type;
        self.begin_scope();
        for param in parameters {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(body)?;
        self.end_scope();
        self.current_function = enclosing_fn;
        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<(), ReefError> {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i)?;
                return Ok(());
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

    fn declare(&mut self, name: &Token) -> Result<(), ReefError> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        let scope = self.scopes.last_mut().expect("expect scope to exist");
        if scope.contains_key(&name.lexeme) {
            return Err(ReefError::reef_error_at_line(
                name,
                "Already a variable with this name in this scope",
            ));
        }
        scope.insert(name.lexeme.to_string(), false);
        Ok(())
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
