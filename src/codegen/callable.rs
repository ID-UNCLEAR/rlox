use crate::codegen::interpreter::{Interpreter, Value};
use crate::codegen::runtime_error::RuntimeError;
use crate::common::Token;

pub trait Callable {
    fn arity(&self) -> usize;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
        token: &Token,
    ) -> Result<Value, RuntimeError>;

    fn to_string(&self) -> String;
}
