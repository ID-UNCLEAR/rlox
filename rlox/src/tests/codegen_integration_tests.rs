#[cfg(test)]
mod codegen_integration_tests {
    use crate::ast::Expr;
    use crate::codegen::interpreter;
    use crate::codegen::interpreter::Value;
    use crate::common::{Literal, Token, TokenType};

    #[test]
    fn test_literal() {
        // Arrange
        let expected: Value = Value::String(String::from("foobar"));
        let expr: Expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::String("foo".into()),
            }),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".into(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal {
                value: Literal::String("bar".into()),
            }),
        };

        // Act
        let result: Value = interpreter::evaluate(&expr);

        // Assert
        assert_eq!(result, expected);
    }
}
