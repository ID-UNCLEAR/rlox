use crate::ast::Expr;
use crate::common::Literal;
use crate::common::TokenType;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

pub fn evaluate(expr: &Expr) -> Value {
    match expr {
        Expr::Literal { value } => match value {
            Literal::Number(n) => Value::Number(*n),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Nil => Value::Nil,
        },

        Expr::Grouping { expression } => evaluate(expression),

        Expr::Unary { operator, right } => {
            let right_val = evaluate(right);
            match operator.token_type {
                TokenType::Minus => match right_val {
                    Value::Number(n) => Value::Number(-n),
                    _ => panic!("Operator token type mismatch"),
                },
                TokenType::Bang => Value::Boolean(!is_truthy(&right_val)),
                _ => panic!("Unknown unary operator"),
            }
        }

        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left_val = evaluate(left);
            let right_val = evaluate(right);

            match operator.token_type {
                TokenType::Plus => match (left_val, right_val) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                    (Value::String(x), Value::String(y)) => Value::String(format!("{}{}", x, y)),
                    _ => panic!("Operands must be two numbers or strings"),
                },
                TokenType::Minus => num_bin_op(left_val, right_val, |x, y| x - y),
                TokenType::Star => num_bin_op(left_val, right_val, |x, y| x * y),
                TokenType::Slash => num_bin_op(left_val, right_val, |x, y| x / y),

                TokenType::Greater => bool_bin_op(left_val, right_val, |x, y| x > y),
                TokenType::GreaterEqual => bool_bin_op(left_val, right_val, |x, y| x >= y),
                TokenType::Less => bool_bin_op(left_val, right_val, |x, y| x < y),
                TokenType::LessEqual => bool_bin_op(left_val, right_val, |x, y| x <= y),
                TokenType::EqualEqual => Value::Boolean(left_val == right_val),
                TokenType::BangEqual => Value::Boolean(left_val != right_val),

                _ => panic!("Unknown binary operator"),
            }
        }
    }
}

fn num_bin_op<F>(x: Value, y: Value, op: F) -> Value
where
    F: Fn(f64, f64) -> f64,
{
    if let (Value::Number(x), Value::Number(y)) = (x, y) {
        Value::Number(op(x, y))
    } else {
        panic!("Operands must be numbers/integers");
    }
}

fn bool_bin_op<F>(x: Value, y: Value, op: F) -> Value
where
    F: Fn(f64, f64) -> bool,
{
    if let (Value::Number(x), Value::Number(y)) = (x, y) {
        Value::Boolean(op(x, y))
    } else {
        panic!("Operands must be numbers/integers");
    }
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Nil => false,
        Value::Boolean(b) => *b,
        _ => true,
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
        let result = evaluate(&expr);

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
        let result = evaluate(&expr);

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
        let result = evaluate(&expr);

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
        let result = evaluate(&expr);

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
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_addition_numbers() {
        // Arrange
        let expr = new_binary_expression(2.0, TokenType::Plus, 3.0);

        // Act
        let result = evaluate(&expr);

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
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::String("Hello, world!".into()));
    }

    #[test]
    #[should_panic(expected = "Operands must be two numbers or strings")]
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

        // Act & Assert
        evaluate(&expr);
    }

    #[test]
    fn binary_subtraction_numbers() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::Minus, 2.0);

        // Act
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    #[should_panic(expected = "Operands must be numbers/integers")]
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

        // Act & Assert
        evaluate(&expr);
    }

    #[test]
    fn binary_multiplication_numbers() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::Star, 2.0);

        // Act
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Number(6.0));
    }

    #[test]
    fn binary_division_numbers() {
        // Arrange
        let expr = new_binary_expression(6.0, TokenType::Slash, 3.0);

        // Act
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn binary_division_by_zero() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::Slash, 0.0);

        // Act
        let result = evaluate(&expr);

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
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_not_equal() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::BangEqual, 2.0);

        // Act
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_greater() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::Greater, 1.0);

        // Act
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_greater_equal() {
        // Arrange
        let expr = new_binary_expression(3.0, TokenType::GreaterEqual, 3.0);

        // Act
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_lesser() {
        // Arrange
        let expr = new_binary_expression(1.0, TokenType::Less, 3.0);

        // Act
        let result = evaluate(&expr);

        // Assert
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn binary_comparison_lesser_equal() {
        // Arrange
        let expr = new_binary_expression(1.0, TokenType::LessEqual, 1.0);

        // Act
        let result = evaluate(&expr);

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
