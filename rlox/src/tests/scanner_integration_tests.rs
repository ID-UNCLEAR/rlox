use crate::Scanner;
use crate::common::{Token, TokenType};
#[cfg(test)]
#[test]
fn test_scanner_basic() {
    // Arrange
    let scanner: Scanner = Scanner::new("");

    // Act
    let tokens: Vec<Token> = scanner.scan_tokens();

    // Assert
    assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
}
