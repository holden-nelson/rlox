use std::fmt;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<TokenLiteral>,
    pub line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: usize) -> Self {
        Self { token_type, lexeme: lexeme.to_owned(), literal: None, line }
    }

    pub fn with_literal(mut self, literal: TokenLiteral) -> Self {
        self.literal = Some(literal);
        self
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            Some(literal) => {
                write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
            }
            None => {
                write!(f, "{:?} {} null", self.token_type, self.lexeme)
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub enum TokenLiteral {
    String(String),
    Number(f64)
}


impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
        }
    }
}