#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One/Two character tokens (operators)
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    If,
    Else,
    True,
    False,
    Fun,
    For,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    // End of File
    Eof,
}
