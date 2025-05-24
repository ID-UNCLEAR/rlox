use crate::codegen::callable::Callable;
use crate::codegen::interpreter::{Interpreter, Value};
use crate::codegen::runtime_error::RuntimeError;
use crate::common::Token;
use std::time::SystemTime;

pub struct Clock {}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
        _token: &Token,
    ) -> Result<Value, RuntimeError> {
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        Ok(Value::Number(now))
    }

    fn to_string(&self) -> String {
        String::from("<native function>")
    }
}
