use std::collections::HashMap;
use std::sync::OnceLock;

use crate::token_type::TokenType;

static KEYWORDS_MAP: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

const KEYWORDS: [(&str, TokenType); 16] = [
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
];

pub fn keywords() -> &'static HashMap<&'static str, TokenType> {
    KEYWORDS_MAP.get_or_init(|| HashMap::from(KEYWORDS))
}
