use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::{
    ExprKind, Token, Value,
    error::ReefError,
    expr::Expr,
    func::{ReefCallable, ReefFunction},
    interpreter::Interpreter,
};

#[derive(Debug, Clone)]
pub enum ClassKind {
    None,
    Class,
}
#[derive(Debug, Clone)]
pub struct ReefClass {
    pub name: String,
    pub methods: HashMap<String, ReefFunction>,
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
    pub fn new(name: String, methods: HashMap<String, ReefFunction>) -> Self {
        ReefClass { name, methods }
    }
    pub fn find_method(&self, name: &str) -> Option<&ReefFunction> {
        self.methods.get(name)
    }
}

impl ReefCallable for ReefClass {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, ReefError> {
        let instance = ReefInstance::new(self.clone());
        if let Some(initializer) = self.find_method("init") {
            initializer
                .bind(Rc::clone(&instance))
                .call(interpreter, arguments)?;
        }

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
    pub fn get(self: &Rc<Self>, name: &Token) -> Result<Value, ReefError> {
        let fields = self.fields.borrow();
        if let Some(value) = fields.get(&name.lexeme) {
            return Ok(value.clone());
        };

        let methods = self.class.borrow();
        if let Some(method) = methods.find_method(&name.lexeme) {
            let bound = method.bind(Rc::clone(self));
            return Ok(Value::Callable(Rc::new(bound)));
        }

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
