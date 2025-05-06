use crate::token_type::TokenType;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_str: String = match &self.literal {
            Some(Literal::String(s)) => s.clone(),
            Some(Literal::Number(n)) => n.to_string(),
            Some(Literal::Boolean(b)) => format!("#{}", b),
            Some(Literal::Nil) => String::from("nil"),
            None => String::from("None"),
        };

        write!(
            f,
            "Line {}. TokenType: `{:?}`, Lexeme: '{}', Literal: {}",
            self.line, self.token_type, self.lexeme, literal_str
        )
    }
}
