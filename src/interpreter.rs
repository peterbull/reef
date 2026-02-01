use crate::{
    Literal, Token, TokenType,
    environment::Environment,
    error::ReefError,
    expr::{ExprKind, Value},
    stmt::StmtKind,
};
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
pub struct Interpreter {
    environment: Environment,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(None),
        }
    }
    pub fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => n.to_string(),
            Value::Boolean(n) => n.to_string(),
            Value::String(n) => n.to_string(),
            Value::Nil => String::from("nil"),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: &ExprKind,
        operator: &Token,
        right: &ExprKind,
    ) -> Result<Value, ReefError> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;
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
    fn evaluate_unary(&mut self, operator: &Token, right: &ExprKind) -> Result<Value, ReefError> {
        let right_val = self.evaluate(right)?;
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

    fn evaluate_literal(&self, value: &Literal) -> Result<Value, ReefError> {
        Ok(match value {
            Literal::String(s) => Value::String(s.clone()),
            Literal::Number(n) => Value::Number(*n),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Nil => Value::Nil,
        })
    }
    fn evaluate_assignment(&mut self, name: &Token, value: &ExprKind) -> Result<Value, ReefError> {
        let value = self.evaluate(value)?;
        self.environment.assign(name, value)
    }
    fn evaluate_variable(&self, name: &Token) -> Result<Value, ReefError> {
        self.environment.get(name)
    }

    pub fn evaluate(&mut self, expr: &ExprKind) -> Result<Value, ReefError> {
        match expr {
            ExprKind::Assign { name, value } => self.evaluate_assignment(name, value),
            ExprKind::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
            // ExprKind::Call {
            //     callee,
            //     token,
            //     arguments,
            // } => {}
            // ExprKind::Get { object, name } => {}
            ExprKind::Grouping { expression } => self.evaluate(expression),
            ExprKind::Literal { value } => self.evaluate_literal(value),
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
            ExprKind::Unary { operator, right } => self.evaluate_unary(operator, right),
            ExprKind::Variable { name } => self.evaluate_variable(name),
            _ => todo!(),
        }
    }

    fn execute_expression(&mut self, expr: &ExprKind) -> Result<(), ReefError> {
        self.evaluate(expr);
        Ok(())
    }

    fn execute_print(&mut self, expr: &ExprKind) -> Result<(), ReefError> {
        let value = self.evaluate(expr)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn execute_var(&mut self, name: &Token, initializer: &ExprKind) -> Result<(), ReefError> {
        let mut value = Value::Nil;
        match initializer {
            ExprKind::None => {}
            _ => {
                value = self.evaluate(initializer)?;
            }
        }
        self.environment
            .define(name.lexeme.clone(), value.clone())?;
        Ok(())
    }

    fn execute_block(
        &mut self,
        statements: &Vec<StmtKind>,
        environment: Environment,
    ) -> Result<(), ReefError> {
        let previous = self.environment.clone();
        self.environment = environment;
        for stmt in statements {
            if let Err(e) = self.execute(stmt) {
                self.environment = previous;
                return Err(e);
            }
        }
        self.environment = previous;
        Ok(())
    }

    pub fn execute(&mut self, stmt: &StmtKind) -> Result<(), ReefError> {
        match stmt {
            StmtKind::Expression { expr } => self.execute_expression(expr)?,
            StmtKind::Print { expr } => self.execute_print(expr)?,
            StmtKind::Var { name, initializer } => self.execute_var(name, initializer)?,
            StmtKind::Block { statements } => {
                self.execute_block(statements, Environment::new(Some(self.environment.clone())))?
            }
            _ => todo!(),
        };
        Ok(())
    }

    pub fn interpret(&mut self, stmts: Vec<StmtKind>) -> Result<(), ReefError> {
        for stmt in stmts {
            self.execute(&stmt)?
        }
        Ok(())
    }
}
impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}
