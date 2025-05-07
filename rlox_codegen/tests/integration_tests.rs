use common::token::{Literal, Token};
use common::token_type::TokenType;
use rlox_ast::expr::Expr;
use rlox_codegen::interpreter::evaluate;

#[test]
fn test_literal() {
    // Arrange
    let expected: Literal = Literal::String(String::from("foobar"));
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
    let result: Literal = evaluate(&expr);

    // Assert
    assert_eq!(result, expected);
}
