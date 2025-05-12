use crate::ast::Expr;
use crate::common::Literal;
use crate::common::TokenType;

pub fn evaluate(expr: &Expr) -> Literal {
    match expr {
        // If the expression is a literal, return its value
        Expr::Literal { value } => (*value).clone(),

        // If the expression is a group (), evaluate the inner expression
        Expr::Grouping { expression } => evaluate(expression),

        // Unary expressions, for example: -x or !x
        Expr::Unary { operator, right } => {
            let right_val: Literal = evaluate(right);
            match operator.token_type {
                // Negation, x = 5, -x means x = -5
                TokenType::Minus => match right_val {
                    Literal::Number(n) => Literal::Number(-n),
                    _ => panic!("Operator token type mismatch"),
                },
                // Logical NOT: x = true, !x => false
                TokenType::Bang => Literal::Boolean(!is_truthy(&right_val)),
                _ => panic!("Unknown unary operator"),
            }
        }

        // Binary expressions, for example: x + y or x > y
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left_val: Literal = evaluate(left);
            let right_val: Literal = evaluate(right);

            match operator.token_type {
                TokenType::Plus => match (left_val, right_val) {
                    (Literal::Number(x), Literal::Number(y)) => Literal::Number(x + y),
                    (Literal::String(x), Literal::String(y)) => {
                        Literal::String(format!("{}{}", x, y))
                    }
                    _ => panic!("Operands must be two numbers or strings"),
                },

                // Binary arithmetic
                TokenType::Minus => num_bin_op(left_val, right_val, |x, y| x - y),
                TokenType::Star => num_bin_op(left_val, right_val, |x, y| x * y),
                TokenType::Slash => num_bin_op(left_val, right_val, |x, y| x / y),

                // Binary comparison TODO: Implement greater than or equal to and stuff
                TokenType::Greater => bool_bin_op(left_val, right_val, |x, y| x > y),
                TokenType::GreaterEqual => bool_bin_op(left_val, right_val, |x, y| x >= y),
                TokenType::Less => bool_bin_op(left_val, right_val, |x, y| x < y),
                TokenType::LessEqual => bool_bin_op(left_val, right_val, |x, y| x <= y),
                TokenType::EqualEqual => bool_bin_op(left_val, right_val, |x, y| x == y),
                TokenType::BangEqual => bool_bin_op(left_val, right_val, |x, y| x != y),

                _ => panic!("Unknown binary operator"),
            }
        }
    }
}

// Generic function that takes a closure and performs the corresponding binary operation, retuning a number/integer
fn num_bin_op<F>(x: Literal, y: Literal, op: F) -> Literal
where
    F: Fn(f64, f64) -> f64,
{
    if let (Literal::Number(x), Literal::Number(y)) = (x, y) {
        Literal::Number(op(x, y))
    } else {
        panic!("Operands must be numbers/integers");
    }
}

// Generic function that takes a closure and performs the corresponding binary operation, returning a bool
fn bool_bin_op<F>(x: Literal, y: Literal, op: F) -> Literal
where
    F: Fn(f64, f64) -> bool,
{
    if let (Literal::Number(x), Literal::Number(y)) = (x, y) {
        Literal::Boolean(op(x, y))
    } else {
        panic!("Operands must be numbers/integers");
    }
}

// Determines whether a given literal value is truthy.
fn is_truthy(literal: &Literal) -> bool {
    match literal {
        Literal::Nil => false,
        Literal::Boolean(b) => *b,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Token;

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
        const VALUE: Literal = Literal::Number(42.0);
        let expr: Expr = Expr::Literal { value: VALUE };

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, VALUE);
    }

    #[test]
    fn grouping_evaluation() {
        // Arrange
        const VALUE: Literal = Literal::Boolean(true);
        let expr: Expr = Expr::Grouping {
            expression: Box::new(Expr::Literal { value: VALUE }),
        };

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, VALUE);
    }

    #[test]
    fn unary_negation() {
        // Arrange
        let expr: Expr = Expr::Unary {
            operator: dummy_token(TokenType::Minus),
            right: Box::new(Expr::Literal {
                value: Literal::Number(5.0),
            }),
        };

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, Literal::Number(-5.0));
    }

    // Examples: x = true, !x => false
    #[test]
    fn unary_not() {
        // Arrange
        let expr: Expr = Expr::Unary {
            operator: dummy_token(TokenType::Bang),
            right: Box::new(Expr::Literal {
                value: Literal::Boolean(true),
            }),
        };

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, Literal::Boolean(false));
    }

    #[test]
    fn unary_not_nil() {
        // Arrange
        let expected: Literal = Literal::Boolean(true);
        let expr: Expr = Expr::Unary {
            operator: dummy_token(TokenType::Bang),
            right: Box::new(Expr::Literal {
                value: Literal::Nil,
            }),
        };

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_addition_numbers() {
        // Arrange
        let expected: Literal = Literal::Number(5.0);
        let expr: Expr = new_binary_expression(2.0, TokenType::Plus, 3.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_addition_strings() {
        // Arrange
        let expected: Literal = Literal::String(String::from("Hello, world!"));
        let expr: Expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::String(String::from("Hello,")),
            }),
            operator: dummy_token(TokenType::Plus),
            right: Box::new(Expr::Literal {
                value: Literal::String(String::from(" world!")),
            }),
        };

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Operands must be two numbers or strings")]
    fn binary_addition_mixed_types() {
        // Arrange
        let expr: Expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::String(String::from("Hello")),
            }),
            operator: dummy_token(TokenType::Plus),
            right: Box::new(Expr::Literal {
                value: Literal::Number(3.0),
            }),
        };

        // Act, Assert
        evaluate(&expr);
    }

    #[test]
    fn binary_subtraction_numbers() {
        // Arrange
        let expected: Literal = Literal::Number(1.0);
        let expr: Expr = new_binary_expression(3.0, TokenType::Minus, 2.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Operands must be numbers/integers")]
    fn binary_subtraction_mixed_types() {
        // Arrange
        let expr: Expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::String("Hello".into()),
            }),
            operator: dummy_token(TokenType::Minus),
            right: Box::new(Expr::Literal {
                value: Literal::Number(3.0),
            }),
        };

        // Act, Assert
        evaluate(&expr);
    }

    #[test]
    fn binary_multiplication_numbers() {
        // Arrange
        let expected: Literal = Literal::Number(6.0);
        let expr: Expr = new_binary_expression(3.0, TokenType::Star, 2.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_division_numbers() {
        // Arrange
        let expected: Literal = Literal::Number(2.0);
        let expr: Expr = new_binary_expression(6.0, TokenType::Slash, 3.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_division_by_zero() {
        // Arrange
        let expr: Expr = new_binary_expression(3.0, TokenType::Slash, 0.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        match result {
            Literal::Number(x) => assert!(x.is_infinite()),
            _ => unreachable!(),
        }
    }

    #[test]
    fn binary_comparison_equal() {
        // Arrange
        let expected: Literal = Literal::Boolean(true);
        let expr: Expr = new_binary_expression(2.0, TokenType::EqualEqual, 2.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_comparison_not_equal() {
        // Arrange
        let expected: Literal = Literal::Boolean(true);
        let expr: Expr = new_binary_expression(3.0, TokenType::BangEqual, 2.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_comparison_greater() {
        // Arrange
        let expected: Literal = Literal::Boolean(true);
        let expr: Expr = new_binary_expression(3.0, TokenType::Greater, 1.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_comparison_greater_equal() {
        // Arrange
        let expected: Literal = Literal::Boolean(true);
        let expr: Expr = new_binary_expression(3.0, TokenType::GreaterEqual, 3.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_comparison_lesser() {
        // Arrange
        let expected: Literal = Literal::Boolean(true);
        let expr: Expr = new_binary_expression(1.0, TokenType::Less, 3.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn binary_comparison_lesser_equal() {
        // Arrange
        let expected: Literal = Literal::Boolean(true);
        let expr: Expr = new_binary_expression(1.0, TokenType::LessEqual, 1.0);

        // Act
        let result: Literal = evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_truthy() {
        assert_eq!(is_truthy(&Literal::Boolean(true)), true);
        assert_eq!(is_truthy(&Literal::Boolean(false)), false);
        assert_eq!(is_truthy(&Literal::Nil), false);
        assert_eq!(is_truthy(&Literal::String("hi".into())), true);
        assert_eq!(is_truthy(&Literal::Number(0.0)), true);
    }
}
