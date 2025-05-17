use crate::common::error_context::ErrorContext;
use crate::common::keywords::keywords;
use crate::common::{Literal, Token, TokenType};
use crate::scanner::scan_error::ScanError;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(mut self) -> Option<Vec<Token>> {
        let mut has_error = false;

        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                has_error = true;
                eprintln!("{}", e);
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        if has_error { None } else { Some(self.tokens) }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), ScanError> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_next_char('*') {
                    let comment_start_line = self.line;

                    // Start of multi-line comment
                    while !(self.is_at_end() || self.peek() == '*' && self.peek_next() == '/') {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }

                    // Consume the '*/' if found
                    if !self.is_at_end() {
                        self.advance(); // consume '*'
                        self.advance(); // consume '/'
                    } else {
                        return Err(self
                            .error_at_line("Unterminated multi-line comment", comment_start_line));
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1,
            '"' => self.string()?,
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphanumeric() || c == '_' => self.identifier(),
            _ => return Err(self.error_at_current(format!("Unexpected character {}", c))),
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect("Cannot advance past source");

        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        });
    }

    fn match_next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<(), ScanError> {
        let start_line = self.line;

        // Trying to find the end of the string
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(self.error_at_line("Unterminated string", start_line));
        }

        // Get the closing "
        self.advance();

        // Trim the surrounding quotes of the value
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::String, Some(Literal::String(value)));

        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part '.'
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Get the value and parse it as a string.
        let text = &self.source[self.start..self.current];
        let value = text.parse::<f64>().expect("Failed to parse number");

        self.add_token_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = keywords()
            .get(text)
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }

    fn error_at_current(&self, message: impl Into<String>) -> ScanError {
        let line_text = self.get_line_text(self.line);
        let lexeme = self.source[self.start..self.current.min(self.source.len())].to_string();

        ScanError {
            message: message.into(),
            context: ErrorContext {
                line_number: self.line,
                line: line_text,
                lexeme,
            },
        }
    }

    fn error_at_line(&self, message: impl Into<String>, line: usize) -> ScanError {
        let line_text = self.get_line_text(line);
        let lexeme = self.source[self.start..self.current.min(self.source.len())].to_string();

        ScanError {
            message: message.into(),
            context: ErrorContext {
                line_number: line,
                line: line_text,
                lexeme,
            },
        }
    }

    fn get_line_text(&self, line_number: usize) -> String {
        self.source
            .lines()
            .nth(line_number - 1)
            .unwrap_or("")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_single_character_tokens() {
        // Arrange
        let source = "(){},.-+;*/";

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        let expected = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::SemiColon,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Eof,
        ];
        let actual: Vec<TokenType> = tokens.iter().map(|t| t.token_type.clone()).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn scan_operators() {
        // Arrange
        let source = "! != = == > >= < <=";

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        let expected = vec![
            TokenType::Bang,
            TokenType::BangEqual,
            TokenType::Equal,
            TokenType::EqualEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Eof,
        ];
        let actual: Vec<TokenType> = tokens.iter().map(|t| t.token_type.clone()).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn scan_comment() {
        // Arrange
        let source = "// This is a comment!";

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_multiline_comment() {
        // Arrange
        let source = "/* This is \n a multiline comment */";

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_unterminated_multiline_comment_returns_none() {
        // Arrange
        let source = "/* This is \n an unterminated multiline comment";

        // Act
        let result = Scanner::new(source).tokenize();

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn scan_string_literal() {
        let source = r#""hello""#;

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].lexeme, r#""hello""#);
        assert_eq!(
            tokens[0].literal,
            Some(Literal::String("hello".to_string()))
        );
    }

    #[test]
    fn scan_unterminated_string_literal_returns_none() {
        // Arrange
        let source = r#""hello"#;

        // Act
        let result = Scanner::new(source).tokenize();

        // Assert
        assert!(result.is_none())
    }

    #[test]
    fn scan_number_literal() {
        // Arrange
        let source = "123.45";

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "123.45");
        assert_eq!(tokens[0].literal, Some(Literal::Number(123.45)));
    }

    #[test]
    fn scan_keywords() {
        // Arrange
        let source = "class MyClass";

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Class);
        assert_eq!(tokens[0].lexeme, "class");
        assert_eq!(tokens[0].literal, None);

        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "MyClass");
        assert_eq!(tokens[1].literal, None);
    }

    #[test]
    fn ignore_whitespace() {
        // Arrange
        let source = " \t\n\r";

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_end_of_file() {
        // Arrange
        let source = "";

        // Act
        let tokens = Scanner::new(source).tokenize().unwrap();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Eof);
        assert_eq!(tokens[0].lexeme, "");
        assert_eq!(tokens[0].literal, None);
    }

    #[test]
    fn scan_invalid_character_returns_none() {
        // Arrange
        let source = "@";

        // Act
        let result = Scanner::new(source).tokenize();

        // Assert
        assert!(result.is_none());
    }
}
