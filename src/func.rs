use crate::environment::{EnvRef, Environment};
use crate::expr::Value;
use crate::stmt::StmtKind;
use crate::{Token, error::ReefError, interpreter::Interpreter};
use std::fmt;
use std::rc::Rc;

pub type InterpreterFn = fn(&mut Interpreter, Vec<Value>) -> Result<Value, ReefError>;

#[derive(Debug, Clone)]
pub enum FunctionKind {
    None,
    Function,
}

#[derive(Debug, Clone)]
pub struct ReefFunction {
    pub declaration: StmtKind,
    pub closure: EnvRef,
}

impl ReefFunction {
    pub fn new(declaration: StmtKind, closure: EnvRef) -> Result<Self, ReefError> {
        match declaration {
            StmtKind::Function { .. } => Ok(ReefFunction {
                declaration,
                closure,
            }),
            _ => Err(ReefError::reef_general_error(
                "expected stmtkind function for reef callable",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub func: InterpreterFn,
}

pub trait ReefCallable: fmt::Debug {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, ReefError>;
    fn name(&self) -> &str;
    fn to_reef_string(&self) -> String {
        format!("<fn {}>", self.name())
    }
}

impl ReefCallable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, ReefError> {
        (self.func)(interpreter, arguments)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl ReefCallable for ReefFunction {
    fn arity(&self) -> usize {
        match &self.declaration {
            StmtKind::Function { parameters, .. } => parameters.len(),
            _ => unreachable!(),
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, ReefError> {
        match &self.declaration {
            StmtKind::Function {
                parameters, body, ..
            } => {
                let env = Environment::new_ref(Some(Rc::clone(&self.closure)));
                for (param, arg) in parameters.iter().zip(arguments) {
                    env.borrow_mut().define(param.lexeme.clone(), arg)?;
                }
                match interpreter.execute_block(body, env) {
                    Err(ReefError::Return(val)) => Ok(val),
                    other => other.map(|_| Value::Nil),
                }
            }
            _ => unreachable!(),
        }
    }

    fn name(&self) -> &str {
        match &self.declaration {
            StmtKind::Function { name, .. } => &name.lexeme,
            _ => unreachable!(),
        }
    }
}
