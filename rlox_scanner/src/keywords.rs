use crate::token_type::TokenType;
use std::collections::HashMap;
use std::sync::OnceLock;

static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

pub fn keywords() -> &'static HashMap<&'static str, TokenType> {
    KEYWORDS.get_or_init(|| {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    })
}
