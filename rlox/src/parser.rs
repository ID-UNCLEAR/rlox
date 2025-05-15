use crate::ast::{Expr, Stmt};
use crate::common::{Literal, Token, TokenType};
use std::fmt;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
    token: Token,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line {}] Error at '{}': {}",
            self.token.line, self.token.lexeme, self.message
        )
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(_) => self.synchronize(),
            }
        }

        statements
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Var]) {
            return self.variable_declaration();
        }

        self.statement()
    }

    fn variable_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .consume(&TokenType::Identifier, "Expected variable name.")?
            .clone();

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            &TokenType::SemiColon,
            "Expected ';' after variable declaration.",
        )?;

        Ok(Stmt::Var {
            name,
            initializer: Box::new(initializer.expect("No initializer found!")),
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        // Print Statement
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }

        // Block statement
        if self.match_token(&[TokenType::LeftBrace]) {
            return self.block_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expect ';' after value.")?;

        Ok(Stmt::Print {
            expression: Box::new(value),
        })
    }

    fn block_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_at_end() && !self.check(&TokenType::RightBrace) {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block statements.")?;

        Ok(Stmt::Block { statements })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expect ';' after expression.")?;

        Ok(Stmt::Expression {
            expression: Box::new(expr),
        })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr: Expr = self.equality()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals: Token = self.previous().clone();
            let value: Expr = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(ParseError {
                message: "Invalid variable assignment".to_string(),
                token: equals,
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(false),
            });
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(true),
            });
        }

        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }

        if self.match_token(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal {
                value: self
                    .previous()
                    .literal
                    .clone()
                    .expect("Expected a literal value"),
            });
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().clone(),
            });
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(self.error("Expected expression"))
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end() && &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&self, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            token: self.peek().clone(),
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                break;
            }

            match self.peek().token_type {
                TokenType::Class => {}
                TokenType::Fun => {}
                TokenType::Var => {}
                TokenType::For => {}
                TokenType::If => {}
                TokenType::While => {}
                TokenType::Print => {}
                TokenType::Return => {}
                _ => {}
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Literal, Token, TokenType};

    fn token(token_type: TokenType, lexeme: &str, literal: Option<Literal>) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line: 1,
        }
    }

    #[test]
    fn whenerror_parsereturnsnone() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Plus, "+", None),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn parse_equality_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Number, "1", Some(Literal::Number(1.0))),
            token(TokenType::EqualEqual, "==", None),
            token(TokenType::Number, "2", Some(Literal::Number(2.0))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected: Expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::Number(1.0),
            }),
            operator: token(TokenType::EqualEqual, "==", None),
            right: Box::new(Expr::Literal {
                value: Literal::Number(2.0),
            }),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_comparison_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Number, "3", Some(Literal::Number(3.0))),
            token(TokenType::Less, "<", None),
            token(TokenType::Number, "4", Some(Literal::Number(4.0))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected: Expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::Number(3.0),
            }),
            operator: token(TokenType::Less, "<", None),
            right: Box::new(Expr::Literal {
                value: Literal::Number(4.0),
            }),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_term_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Number, "5", Some(Literal::Number(5.0))),
            token(TokenType::Plus, "+", None),
            token(TokenType::Number, "6", Some(Literal::Number(6.0))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected: Expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::Number(5.0),
            }),
            operator: token(TokenType::Plus, "+", None),
            right: Box::new(Expr::Literal {
                value: Literal::Number(6.0),
            }),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_factor_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Number, "7", Some(Literal::Number(7.0))),
            token(TokenType::Star, "*", None),
            token(TokenType::Number, "8", Some(Literal::Number(8.0))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::Number(7.0),
            }),
            operator: token(TokenType::Star, "*", None),
            right: Box::new(Expr::Literal {
                value: Literal::Number(8.0),
            }),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_bang_unary_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Bang, "!", None),
            token(TokenType::True, "true", Some(Literal::Boolean(true))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Unary {
            operator: token(TokenType::Bang, "!", None),
            right: Box::new(Expr::Literal {
                value: Literal::Boolean(true),
            }),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_minus_unary_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Minus, "-", None),
            token(TokenType::Number, "3", Some(Literal::Number(3.0))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Unary {
            operator: token(TokenType::Minus, "-", None),
            right: Box::new(Expr::Literal {
                value: Literal::Number(3.0),
            }),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_primary_no_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::LeftParen, "(", None),
            token(TokenType::Number, "42", Some(Literal::Number(42.0))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn parse_primary_true_literal_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::True, "true", Some(Literal::Boolean(true))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Literal {
            value: Literal::Boolean(true),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_primary_false_literal_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::False, "false", Some(Literal::Boolean(false))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Literal {
            value: Literal::Boolean(false),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_primary_nil_literal_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Nil, "nil", Some(Literal::Nil)),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Literal {
            value: Literal::Nil,
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_primary_string_literal_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(
                TokenType::String,
                "test",
                Some(Literal::String("test".to_string())),
            ),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Literal {
            value: Literal::String("test".to_string()),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_primary_number_literal_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::Number, "123", Some(Literal::Number(123.0))),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Literal {
            value: Literal::Number(123.0),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_primary_grouping_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::LeftParen, "(", None),
            token(TokenType::Number, "1", Some(Literal::Number(1.0))),
            token(TokenType::RightParen, ")", None),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Literal::Number(1.0),
            }),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }

    #[test]
    fn parse_primary_nested_grouping_expression() {
        // Arrange
        let tokens: Vec<Token> = vec![
            token(TokenType::LeftParen, "(", None),
            token(TokenType::LeftParen, "(", None),
            token(TokenType::Number, "1", Some(Literal::Number(1.0))),
            token(TokenType::RightParen, ")", None),
            token(TokenType::RightParen, ")", None),
            token(TokenType::Eof, "", None),
        ];
        let mut parser: Parser = Parser::new(tokens);

        // Act
        let result = parser.parse();

        // Assert
        let expected = Expr::Grouping {
            expression: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Literal::Number(1.0),
                }),
            }),
        };

        match &result[0] {
            Stmt::Expression { expression } => assert_eq!(**expression, expected),
            _ => panic!("Expected expression statement."),
        }
    }
}
