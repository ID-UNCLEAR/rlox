#[cfg(test)]
mod scanner_integration_tests {
    use crate::common::TokenType;
    use crate::scanner::Scanner;

    #[test]
    fn test_scanner_basic() {
        // Arrange
        let scanner = Scanner::new("");

        // Act
        let tokens = scanner.tokenize().unwrap();

        // Assert
        assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
    }
}
