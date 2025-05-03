use crate::keywords::keywords;
use crate::token::{Literal, Token};
pub use crate::token_type::TokenType;

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

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

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
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphanumeric() || c == '_' => self.identifier(),
            _ => panic!("Unexpected character '{}' on line {}", c, self.line),
        }
    }

    fn advance(&mut self) -> char {
        let c: char = self
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
        let text: String = self.source[self.start..self.current].to_string();
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

    fn string(&mut self) {
        // Trying to find the end of the string
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string at line {}", self.line);
        }

        // Get the closing "
        self.advance();

        // Trim the surrounding quotes of the value
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::String, Some(Literal::String(value)));
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
        let text: &str = &self.source[self.start..self.current];
        let value: f64 = text.parse::<f64>().expect("Failed to parse number");

        self.add_token_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text: &str = &self.source[self.start..self.current];
        let token_type: TokenType = keywords()
            .get(text)
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_single_character_tokens() {
        // Arrange
        let source: &str = "(){},.-+;/*";

        // Act
        let tokens: Vec<Token> = Scanner::new(source).scan_tokens();

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
            TokenType::Slash,
            TokenType::Star,
            TokenType::Eof,
        ];
        let actual: Vec<TokenType> = tokens.iter().map(|t| t.token_type.clone()).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn scan_operators() {
        // Arrange
        let source: &str = "! != = == > >= < <=";

        // Act
        let tokens: Vec<Token> = Scanner::new(source).scan_tokens();

        // Assert
        let expected: Vec<TokenType> = vec![
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
        let source: &str = "// This is a comment!";

        // Act
        let tokens: Vec<Token> = Scanner::new(source).scan_tokens();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_string_literal() {
        let source: &str = r#""hello""#;

        // Act
        let tokens: Vec<Token> = Scanner::new(source).scan_tokens();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].lexeme, r#""hello""#);
        assert_eq!(
            tokens[0].literal,
            Some(Literal::String("hello".to_string()))
        );
    }

    #[test]
    #[should_panic(expected = "Unterminated string")]
    fn scan_unterminated_string_literal_panics() {
        // Arrange, Act
        Scanner::new(r#""hello"#).scan_tokens();
    }

    #[test]
    fn scan_number_literal() {
        // Arrange
        let source: &str = "123.45";

        // Act
        let tokens: Vec<Token> = Scanner::new(source).scan_tokens();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "123.45");
        assert_eq!(tokens[0].literal, Some(Literal::Number(123.45)));
    }

    #[test]
    fn scan_keywords() {
        // Arrange
        let source: &str = "class MyClass";

        // Act
        let tokens: Vec<Token> = Scanner::new(source).scan_tokens();

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
        let source: &str = " \t\n\r";

        // Act
        let tokens: Vec<Token> = Scanner::new(source).scan_tokens();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_end_of_file() {
        // Arrange
        let source: &str = "";

        // Act
        let tokens: Vec<Token> = Scanner::new(source).scan_tokens();

        // Assert
        assert_eq!(tokens[0].token_type, TokenType::Eof);
        assert_eq!(tokens[0].lexeme, "");
        assert_eq!(tokens[0].literal, None);
    }

    #[test]
    #[should_panic(expected = "Unexpected character")]
    fn scan_invalid_character_panics() {
        // Arrange, Act
        Scanner::new("@").scan_tokens();
    }
}
