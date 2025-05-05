use common::token_type::TokenType;
use rlox_scanner::scanner::Scanner;

#[test]
fn test_scanner_basic() {
    // Arrange
    let scanner = Scanner::new("");

    // Act
    let tokens = scanner.scan_tokens();

    // Assert - simple check: last token should be Eof
    assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
}
