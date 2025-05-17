use crate::codegen::interpreter::Value;
use crate::codegen::runtime_error::RuntimeError;
use crate::common::Token;
use crate::common::error_context::ErrorContext;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    const UNDEFINED_VARIABLE: &'static str = "undefined variable";

    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get_value(&self, name: &Token) -> Result<Value, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow().get_value(name)
        } else {
            Err(error(Self::UNDEFINED_VARIABLE.into(), name.clone()))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(error(Self::UNDEFINED_VARIABLE.into(), name.clone()))
        }
    }
}

fn error(message: String, token: Token) -> RuntimeError {
    RuntimeError {
        message,
        context: ErrorContext {
            line_number: token.line,
            line: "".into(),
            lexeme: token.lexeme,
        },
    }
}
