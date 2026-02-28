use crate::environment::{EnvRef, Environment};
use crate::expr::Value;
use crate::stmt::StmtKind;
use crate::{Token, error::ReefError, interpreter::Interpreter};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub type InterpreterFn = fn(&Interpreter, Vec<Value>) -> Result<Value, ReefError>;

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    name: Token,
    parameters: Vec<Token>,
    body: Vec<StmtKind>,
}

impl FunctionDecl {
    pub fn from_statement(stmt: StmtKind) -> Result<Self, ReefError> {
        match &stmt {
            StmtKind::Function {
                name,
                parameters,
                body,
            } => Ok(FunctionDecl {
                name: name.clone(),
                parameters: parameters.clone(),
                body: body.clone(),
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

#[derive(Debug, Clone)]
pub struct ReefFunction {
    pub declaration: FunctionDecl,
    pub closure: EnvRef,
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

impl ReefFunction {
    pub fn new(declaration: StmtKind, closure: EnvRef) -> Result<Self, ReefError> {
        let declaration = FunctionDecl::from_statement(declaration)?;
        Ok(Self {
            declaration,
            closure,
        })
    }
}

impl ReefCallable for ReefFunction {
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, ReefError> {
        let environment = Environment::new_ref(Some(Rc::clone(&self.closure)));

        for (i, param) in self.declaration.parameters.iter().enumerate() {
            environment
                .borrow_mut()
                .define(param.lexeme.clone(), arguments[i].clone())?;
        }

        match interpreter.execute_block(&self.declaration.body, environment) {
            Ok(_) => Ok(Value::Nil),
            Err(e) => match e {
                ReefError::Return(val) => Ok(val),
                _ => Err(e),
            },
        }
    }

    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
}
