use crate::ast::Stmt;
use crate::codegen::callable::Callable;
use crate::codegen::environment::Environment;
use crate::codegen::interpreter::{Interpreter, Value};
use crate::codegen::runtime_error::RuntimeError;
use crate::common::Token;

pub struct Function {
    pub declaration: Stmt,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        match &self.declaration {
            Stmt::Function { parameters, .. } => parameters.len(),
            _ => 0,
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
        _token: &Token,
    ) -> Result<Value, RuntimeError> {
        let environment = Environment::with_enclosing(interpreter.globals.clone());

        if let Stmt::Function {
            parameters, body, ..
        } = &self.declaration
        {
            for (param, arg) in parameters.iter().zip(arguments.iter()) {
                environment
                    .borrow_mut()
                    .define(param.lexeme.clone(), arg.clone());
            }

            interpreter.execute_block(body, environment)?;
        }

        Ok(Value::Nil)
    }

    fn to_string(&self) -> String {
        match &self.declaration {
            Stmt::Function { name, .. } => {
                format!("<fn {}>", name.lexeme)
            }
            _ => "<fn>".to_string(),
        }
    }
}
