use std::fmt;

use crate::{
    ast::{BinaryOperator, UnaryOperator},
    lox::CompileError,
};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<TokenLiteral>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_owned(),
            literal: None,
            line,
        }
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum TokenLiteral {
    String(String),
    Number(f64),
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
        }
    }
}

impl TryFrom<&Token> for BinaryOperator {
    type Error = CompileError;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match &token.token_type {
            TokenType::EqualEqual => Ok(Self::Equals),
            TokenType::BangEqual => Ok(Self::NotEquals),
            TokenType::Less => Ok(Self::LessThan),
            TokenType::LessEqual => Ok(Self::LessThanEqual),
            TokenType::Greater => Ok(Self::GreaterThan),
            TokenType::GreaterEqual => Ok(Self::GreaterThanEqual),
            TokenType::Plus => Ok(Self::Plus),
            TokenType::Minus => Ok(Self::Minus),
            TokenType::Star => Ok(Self::Times),
            TokenType::Slash => Ok(Self::Divide),
            _ => Err(CompileError {
                line: token.line,
                at: token.lexeme.clone(),
                message: "Expected a binary operator.".to_owned(),
            }),
        }
    }
}

impl TryFrom<&Token> for UnaryOperator {
    type Error = CompileError;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match &token.token_type {
            TokenType::Minus => Ok(Self::Negate),
            TokenType::Bang => Ok(Self::Not),
            _ => Err(CompileError {
                line: token.line,
                at: token.lexeme.clone(),
                message: "Expected a unary operator.".to_owned(),
            }),
        }
    }
}
