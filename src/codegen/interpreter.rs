use crate::ast::{Expr, Stmt};
use crate::codegen::callable::Callable;
use crate::codegen::clock::Clock;
use crate::codegen::environment::Environment;
use crate::codegen::function::Function;
use crate::codegen::runtime_error::RuntimeError;
use crate::common::TokenType;
use crate::common::error_context::ErrorContext;
use crate::common::{Literal, Token};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Callable(Rc<dyn Callable>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Callable(c) => write!(f, "{}", c.to_string()),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Nil => write!(f, "Nil"),
            Value::Callable(_) => write!(f, "Callable(<fn>)"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;

        match (self, other) {
            (Number(a), Number(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Boolean(a), Boolean(b)) => a == b,
            (Nil, Nil) => true,
            // You can’t really compare two Callable instances meaningfully,
            // unless you assign each a unique ID or name
            (Callable(_), Callable(_)) => false,
            _ => false,
        }
    }
}

pub struct Interpreter {
    statements: Vec<Stmt>,
    environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        let globals = Environment::new();

        globals
            .borrow_mut()
            .define("clock".into(), Value::Callable(Rc::new(Clock {})));

        Interpreter {
            globals: globals.clone(),
            statements: stmts,
            environment: globals,
        }
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        let stmts = std::mem::take(&mut self.statements);
        for stmt in stmts {
            match self.execute(&stmt) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(e);
                }
            };
        }

        Ok(())
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression { expression: expr } => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print { expression: expr } => {
                let value = self.evaluate(expr)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Function {
                name,
                parameters: _parameters,
                body: _body,
            } => {
                let function = Function {
                    declaration: stmt.clone(),
                };

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Value::Callable(Rc::new(function)));

                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let value = if let Some(expr) = initializer {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);

                Ok(())
            }
            Stmt::Block { statements } => {
                let new_env = Environment::with_enclosing(self.environment.clone());
                self.execute_block(statements, Environment::with_enclosing(new_env))
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate(condition.as_ref())?) {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }

                Ok(())
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.evaluate(condition.as_ref())?) {
                    self.execute(body)?;
                }

                Ok(())
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Assign { name, value } => {
                let val = self.evaluate(value)?;
                self.environment.borrow_mut().assign(name, val.clone())?;
                Ok(val)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Plus => match (left_val, right_val) {
                        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
                        (Value::String(x), Value::String(y)) => {
                            Ok(Value::String(format!("{}{}", x, y)))
                        }
                        _ => Err(error("Operator token type mismatch".into(), operator)),
                    },
                    TokenType::Minus => num_bin_op(left_val, right_val, |x, y| x - y)
                        .map_err(|msg| error(msg, operator)),

                    TokenType::Star => num_bin_op(left_val, right_val, |x, y| x * y)
                        .map_err(|msg| error(msg, operator)),

                    TokenType::Slash => num_bin_op(left_val, right_val, |x, y| x / y)
                        .map_err(|msg| error(msg, operator)),

                    TokenType::Greater => bool_bin_op(left_val, right_val, |x, y| x > y)
                        .map_err(|msg| error(msg, operator)),
                    TokenType::GreaterEqual => bool_bin_op(left_val, right_val, |x, y| x >= y)
                        .map_err(|msg| error(msg, operator)),

                    TokenType::Less => bool_bin_op(left_val, right_val, |x, y| x < y)
                        .map_err(|msg| error(msg, operator)),
                    TokenType::LessEqual => bool_bin_op(left_val, right_val, |x, y| x <= y)
                        .map_err(|msg| error(msg, operator)),

                    TokenType::EqualEqual => Ok(Value::Boolean(left_val == right_val)),
                    TokenType::BangEqual => Ok(Value::Boolean(left_val != right_val)),

                    _ => Err(error("Operator token type mismatch".into(), operator)),
                }
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(callee)?;

                let mut args = Vec::new();
                for argument in arguments {
                    args.push(self.evaluate(argument)?);
                }

                if let Value::Callable(function) = callee {
                    if args.len() != function.arity() {
                        Err(error(
                            format!(
                                "expected {} arguments but received {}",
                                function.arity(),
                                arguments.len()
                            ),
                            paren,
                        ))?;
                    }

                    function.call(self, args, paren)
                } else {
                    Err(error("callee must be a function".into(), paren))
                }
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => match value {
                Literal::Number(n) => Ok(Value::Number(*n)),
                Literal::String(s) => Ok(Value::String(s.clone())),
                Literal::Boolean(b) => Ok(Value::Boolean(*b)),
                Literal::Nil => Ok(Value::Nil),
            },
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left)?;

                match operator.token_type {
                    TokenType::Or => {
                        if is_truthy(&left_val) {
                            return Ok(left_val);
                        }
                    }
                    TokenType::And => {
                        if !is_truthy(&left_val) {
                            return Ok(left_val);
                        }
                    }
                    _ => {}
                }

                let right_val = self.evaluate(right)?;
                Ok(right_val)
            }
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => match right_val {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(error("Operator token type mismatch".into(), operator)),
                    },
                    TokenType::Bang => Ok(Value::Boolean(!is_truthy(&right_val))),
                    _ => Err(error("Operator token type mismatch".into(), operator)),
                }
            }
            Expr::Variable { name } => self.environment.borrow().get_value(name),
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        // Save the previous environment
        let previous = self.environment.clone();

        // Switch to the new environment (the block scope)
        self.environment = environment;

        // Execute all statements inside the block
        for stmt in statements {
            self.execute(stmt)?;
        }

        // Restore the previous environment (outer scope)
        self.environment = previous;

        Ok(())
    }
}

fn num_bin_op<F>(x: Value, y: Value, op: F) -> Result<Value, String>
where
    F: Fn(f64, f64) -> f64,
{
    if let (Value::Number(x), Value::Number(y)) = (x, y) {
        Ok(Value::Number(op(x, y)))
    } else {
        Err("Operator token type mismatch".into())
    }
}

fn bool_bin_op<F>(x: Value, y: Value, op: F) -> Result<Value, String>
where
    F: Fn(f64, f64) -> bool,
{
    if let (Value::Number(x), Value::Number(y)) = (x, y) {
        Ok(Value::Boolean(op(x, y)))
    } else {
        Err("Operands must be numbers/integers".into())
    }
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Nil => false,
        Value::Boolean(b) => *b,
        _ => true,
    }
}

fn error(message: String, token: &Token) -> RuntimeError {
    RuntimeError {
        message,
        context: ErrorContext {
            line_number: token.line,
            lexeme: token.lexeme.clone(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Literal, Token, TokenType};

    fn dummy_token(token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: "".into(),
            literal: None,
            line: 1,
        }
    }

    fn new_binary_expression(left_value: f64, token_type: TokenType, right_value: f64) -> Expr {
        Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::Number(left_value),
            }),
            operator: dummy_token(token_type),
            right: Box::new(Expr::Literal {
                value: Literal::Number(right_value),
            }),
        }
    }

    #[test]
    fn literal_evaluation() {
        // Arrange
        let expr = Expr::Literal {
            value: Literal::Number(42.0),
        };

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn grouping_evaluation() {
        // Arrange
        let expr = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Literal::Boolean(true),
            }),
        };

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn unary_negation() {
        // Arrange
        let expr = Expr::Unary {
            operator: dummy_token(TokenType::Minus),
            right: Box::new(Expr::Literal {
                value: Literal::Number(5.0),
            }),
        };

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Number(-5.0));
    }

    #[test]
    fn unary_not() {
        // Arrange
        let expr = Expr::Unary {
            operator: dummy_token(TokenType::Bang),
            right: Box::new(Expr::Literal {
                value: Literal::Boolean(true),
            }),
        };

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn unary_not_nil() {
        // Arrange
        let expr = Expr::Unary {
            operator: dummy_token(TokenType::Bang),
            right: Box::new(Expr::Literal {
                value: Literal::Nil,
            }),
        };

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_addition_numbers() {
        // Arrange
        let expr = new_binary_expression(2.0, TokenType::Plus, 3.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn binary_addition_strings() {
        // Arrange
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::String("Hello,".into()),
            }),
            operator: dummy_token(TokenType::Plus),
            right: Box::new(Expr::Literal {
                value: Literal::String(" world!".into()),
            }),
        };

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::String("Hello, world!".into()));
    }

    #[test]
    fn binary_addition_mixed_types() {
        // Arrange
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::String("Hello".into()),
            }),
            operator: dummy_token(TokenType::Plus),
            right: Box::new(Expr::Literal {
                value: Literal::Number(3.0),
            }),
        };

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn binary_subtraction_numbers() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::Minus, 2.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn binary_subtraction_mixed_types() {
        // Arrange
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::String("Hello".into()),
            }),
            operator: dummy_token(TokenType::Minus),
            right: Box::new(Expr::Literal {
                value: Literal::Number(3.0),
            }),
        };

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn binary_multiplication_numbers() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::Star, 2.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Number(6.0));
    }

    #[test]
    fn binary_division_numbers() {
        // Arrange
        let expr = new_binary_expression(6.0, TokenType::Slash, 3.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn binary_division_by_zero() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::Slash, 0.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        match result {
            Value::Number(x) => assert!(x.is_infinite()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn binary_comparison_equal() {
        // Arrange
        let expr = new_binary_expression(2.0, TokenType::EqualEqual, 2.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_not_equal() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::BangEqual, 2.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_greater() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::Greater, 1.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_greater_equal() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::GreaterEqual, 3.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_lesser() {
        // Arrange
        let expr = new_binary_expression(1.0, TokenType::Less, 3.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_lesser_equal() {
        // Arrange
        let expr = new_binary_expression(1.0, TokenType::LessEqual, 1.0);

        // Act
        let result = Interpreter::new(vec![]).evaluate(&expr).unwrap();

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_is_truthy() {
        assert_eq!(is_truthy(&Value::Boolean(true)), true);
        assert_eq!(is_truthy(&Value::Boolean(false)), false);
        assert_eq!(is_truthy(&Value::Nil), false);
        assert_eq!(is_truthy(&Value::String("hi".into())), true);
        assert_eq!(is_truthy(&Value::Number(0.0)), true);
    }
}
