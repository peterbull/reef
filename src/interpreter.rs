use std::{
    collections::HashMap,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    Literal, Token, TokenType,
    class::{ReefClass, ReefClassAttrs},
    environment::{EnvRef, Environment},
    error::ReefError,
    expr::{Expr, ExprKind, Value},
    stmt::StmtKind,
};
use crate::{
    class::ReefInstance,
    func::{NativeFunction, ReefCallable, ReefFunction},
};

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

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub globals: EnvRef,
    pub environment: EnvRef,
    pub locals: HashMap<*const ExprKind, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new_ref(None);
        let clock = NativeFunction {
            name: "reef_clock".to_string(),
            arity: 0,
            func: |_interpreter, _args| {
                let time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                Ok(Value::Number(time))
            },
        };

        globals
            .borrow_mut()
            .define("clock".to_string(), Value::Callable(Rc::new(clock)))
            .expect("expect clock function to be definable");

        Interpreter {
            environment: Rc::clone(&globals),
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => n.to_string(),
            Value::Boolean(n) => n.to_string(),
            Value::String(n) => n.to_string(),
            Value::Nil => String::from("nil"),
            Value::Callable(n) => n.to_reef_string(),
            Value::Instance(n) => n.to_class_string(),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, ReefError> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Plus => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::Minus => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::Star => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::Slash => match (&left_val, &right_val) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                _ => Err(ReefError::reef_runtime_error(
                    operator,
                    "Binary evaluation error",
                )),
            },
            TokenType::EqualEqual => Ok(Value::Boolean(is_equal(&left_val, &right_val))),
            TokenType::BangEqual => Ok(Value::Boolean(!is_equal(&left_val, &right_val))),
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

    fn evaluate_unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, ReefError> {
        let right_val = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Minus => match right_val {
                Value::Number(n) => Ok(Value::Number(-n)),
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

    fn evaluate_literal(&self, value: &Literal) -> Result<Value, ReefError> {
        Ok(match value {
            Literal::String(s) => Value::String(s.clone()),
            Literal::Number(n) => Value::Number(*n),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Nil => Value::Nil,
        })
    }

    fn evaluate_assignment(
        &mut self,
        name: &Token,
        value: &Expr,
        expr: &Expr,
    ) -> Result<Value, ReefError> {
        let value = self.evaluate(value)?;
        let distance = self.locals.get(&Rc::as_ptr(expr));
        match distance {
            Some(dist) => self
                .environment
                .borrow_mut()
                .assign_at(dist, &name.lexeme, value),
            None => self.globals.borrow_mut().assign(&name.lexeme, value),
        }
    }

    fn evaluate_variable(&mut self, name: &Token, expr: &Expr) -> Result<Value, ReefError> {
        self.lookup_variable(name, expr)
    }

    fn lookup_variable(&mut self, name: &Token, expr: &Expr) -> Result<Value, ReefError> {
        let distance = self.locals.get(&Rc::as_ptr(expr));
        match distance {
            Some(dist) => self.environment.borrow().get_at(dist, &name.lexeme),
            None => self.globals.borrow().get(&name.lexeme),
        }
    }

    fn evaluate_logical(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, ReefError> {
        let left_val = self.evaluate(left)?;
        match operator.token_type {
            TokenType::Or => {
                if left_val.is_truthy() {
                    return Ok(left_val);
                }
            }
            _ => {
                if !left_val.is_truthy() {
                    return Ok(left_val);
                }
            }
        }
        self.evaluate(right)
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, ReefError> {
        match expr.as_ref() {
            ExprKind::Assign { name, value } => self.evaluate_assignment(name, value, expr),
            ExprKind::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
            ExprKind::Call {
                callee,
                token,
                arguments,
            } => self.evaluate_call_expr(callee, token, arguments),
            ExprKind::Grouping { expression } => self.evaluate(expression),
            ExprKind::Literal { value } => self.evaluate_literal(value),
            ExprKind::Logical {
                left,
                operator,
                right,
            } => self.evaluate_logical(left, operator, right),
            ExprKind::Unary { operator, right } => self.evaluate_unary(operator, right),
            ExprKind::Variable { name } => self.evaluate_variable(name, expr),
            ExprKind::None => Ok(Value::Nil),
            ExprKind::Get { name, object } => match self.evaluate(object)? {
                Value::Instance(obj) => obj.get(name),
                _ => Err(ReefError::reef_runtime_error(
                    name,
                    "only instances have properties",
                )),
            },
            ExprKind::Set {
                object,
                name,
                value,
            } => match self.evaluate(object)? {
                Value::Instance(obj) => {
                    let value = self.evaluate(value)?;
                    obj.set(name, value.clone())?;
                    Ok(value)
                }
                _ => Err(ReefError::reef_runtime_error(
                    name,
                    "only instances have properties",
                )),
            },
            ExprKind::This { keyword } => self.lookup_variable(keyword, expr),
            _ => todo!(),
        }
    }

    fn evaluate_call_expr(
        &mut self,
        callee: &Expr,
        token: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<Value, ReefError> {
        let callee_val = self.evaluate(callee)?;
        let mut arguments_val: Vec<Value> = Vec::new();
        for arg in arguments {
            arguments_val.push(self.evaluate(arg)?);
        }
        match callee_val {
            Value::Callable(callable) => {
                let expected_len = callable.arity();
                let actual_len = arguments_val.len();
                if expected_len != actual_len {
                    return Err(ReefError::reef_runtime_error(
                        token,
                        &format!("Expected: {} args, got {} args", expected_len, actual_len),
                    ));
                }
                callable.call(self, arguments_val)
            }
            _ => Err(ReefError::reef_runtime_error(
                token,
                "can only call funcs and classes",
            )),
        }
    }

    fn execute_expression(&mut self, expr: &Expr) -> Result<(), ReefError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn execute_print(&mut self, expr: &Expr) -> Result<(), ReefError> {
        let value = self.evaluate(expr)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn execute_var(&mut self, name: &Token, initializer: &Expr) -> Result<(), ReefError> {
        let value = match initializer.as_ref() {
            ExprKind::None => Value::Nil,
            _ => self.evaluate(initializer)?,
        };
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), value)?;
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<StmtKind>,
        environment: EnvRef,
    ) -> Result<(), ReefError> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;
        let result = (|| {
            for stmt in statements {
                self.execute(stmt)?;
            }
            Ok(())
        })();

        self.environment = previous;
        result
    }

    fn execute_if(
        &mut self,
        condition: &Expr,
        then_branch: &StmtKind,
        else_branch: &Option<Box<StmtKind>>,
    ) -> Result<(), ReefError> {
        if self.evaluate(condition)?.is_truthy() {
            self.execute(then_branch)?;
        } else if let Some(b) = else_branch {
            self.execute(b)?;
        }
        Ok(())
    }

    fn execute_func(&mut self, stmt: StmtKind, name: &Token) -> Result<(), ReefError> {
        let function = ReefFunction::new(stmt.clone(), Rc::clone(&self.environment))?;
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Value::Callable(Rc::new(function)))?;
        Ok(())
    }

    fn execute_while(&mut self, condition: &Expr, body: &StmtKind) -> Result<(), ReefError> {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body)?;
        }
        Ok(())
    }

    pub fn execute(&mut self, stmt: &StmtKind) -> Result<(), ReefError> {
        match stmt {
            StmtKind::Expression { expr } => self.execute_expression(expr)?,
            StmtKind::Print { expr } => self.execute_print(expr)?,
            StmtKind::Var { name, initializer } => self.execute_var(name, initializer)?,
            StmtKind::Block { statements } => {
                let new_env = Environment::new_ref(Some(Rc::clone(&self.environment)));
                self.execute_block(statements, new_env)?
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => self.execute_if(condition, then_branch, else_branch)?,
            StmtKind::While { condition, body } => self.execute_while(condition, body)?,
            StmtKind::Function {
                name,
                parameters: _,
                body: _,
            } => self.execute_func(stmt.clone(), name)?,
            StmtKind::Return { keyword: _, value } => {
                let final_value = self.evaluate(value)?;
                Err(ReefError::reef_return(final_value))?
            }
            StmtKind::Class { name, methods } => {
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.to_string(), Value::Nil)?;
                let mut class_methods: HashMap<String, ReefFunction> = HashMap::new();
                for method in methods {
                    match method {
                        StmtKind::Function { name, .. } => {
                            let is_init = name.lexeme == "init";
                            let mut function =
                                ReefFunction::new(method.clone(), Rc::clone(&self.environment))?;
                            function.is_initializer = is_init;
                            class_methods.insert(name.lexeme.clone(), function);
                        }
                        _ => {}
                    }
                }

                let class = ReefClass::new(name.lexeme.clone(), class_methods);
                let class_val = Value::Callable(Rc::new(class) as Rc<dyn ReefCallable>);

                self.environment
                    .borrow_mut()
                    .assign(&name.lexeme, class_val)?;
            }
            _ => todo!(),
        };
        Ok(())
    }

    pub fn interpret(&mut self, stmts: &[StmtKind]) -> Result<(), ReefError> {
        for stmt in stmts {
            self.execute(stmt)?
        }
        Ok(())
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) -> Result<(), ReefError> {
        self.locals.insert(Rc::as_ptr(expr), depth);
        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}
