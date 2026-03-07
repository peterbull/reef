use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ExprKind, Token, Value, error::ReefError, expr::Expr, func::ReefCallable,
    interpreter::Interpreter,
};

#[derive(Debug, Clone)]
pub struct ReefClass {
    pub name: String,
}

pub trait ReefClassAttrs {
    fn to_class_string(&self) -> String;
}

impl ReefClassAttrs for ReefClass {
    fn to_class_string(&self) -> String {
        self.name.to_string()
    }
}
impl ReefClass {
    pub fn new(name: String) -> Self {
        ReefClass { name }
    }
}

impl ReefCallable for ReefClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, ReefError> {
        let instance = ReefInstance::new(self.clone());
        Ok(Value::Instance(instance))
    }

    fn name(&self) -> &str {
        &self.name
    }
}
pub type ReefClassRef = Rc<RefCell<ReefClass>>;
pub type ReefInstanceRef = Rc<ReefInstance>;
#[derive(Debug, Clone)]
pub struct ReefInstance {
    class: ReefClassRef,
    fields: RefCell<HashMap<String, Value>>,
}
impl ReefInstance {
    pub fn new(class: ReefClass) -> Rc<Self> {
        Rc::new(ReefInstance {
            class: Rc::new(RefCell::new(class)),
            fields: RefCell::new(HashMap::new()),
        })
    }
    pub fn get(&self, name: &Token) -> Result<Value, ReefError> {
        let fields = self.fields.borrow();
        if let Some(value) = fields.get(&name.lexeme) {
            return Ok(value.clone());
        };
        Err(ReefError::reef_error_at_line(name, "Undefined property"))
    }
    pub fn set(&self, name: &Token, value: Value) -> Result<(), ReefError> {
        self.fields
            .borrow_mut()
            .insert(name.lexeme.to_string(), value);
        Ok(())
    }
}
impl ReefClassAttrs for ReefInstance {
    fn to_class_string(&self) -> String {
        format!("{} instance", self.class.borrow().name)
    }
}
