use crate::{
    lox::{
        CompileError,
        LoxError::{self, Compile},
    },
    tokens::{Token, TokenLiteral, TokenType},
};

pub struct Scanner<'src> {
    source: &'src str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token()?;
        }

        self.start = self.current;
        self.add_token(TokenType::Eof);

        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        if let Some(c) = self.advance() {
            match c {
                '(' => {
                    self.add_token(TokenType::LeftParen);
                    return Ok(());
                }
                ')' => {
                    self.add_token(TokenType::RightParen);
                    return Ok(());
                }
                '{' => {
                    self.add_token(TokenType::LeftBrace);
                    return Ok(());
                }
                '}' => {
                    self.add_token(TokenType::RightBrace);
                    return Ok(());
                }
                ',' => {
                    self.add_token(TokenType::Comma);
                    return Ok(());
                }
                '.' => {
                    self.add_token(TokenType::Dot);
                    return Ok(());
                }
                '-' => {
                    self.add_token(TokenType::Minus);
                    return Ok(());
                }
                '+' => {
                    self.add_token(TokenType::Plus);
                    return Ok(());
                }
                ';' => {
                    self.add_token(TokenType::Semicolon);
                    return Ok(());
                }
                '*' => {
                    self.add_token(TokenType::Star);
                    return Ok(());
                }

                '!' => {
                    let token_type = if self.check_next('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(token_type);
                    return Ok(());
                }

                '=' => {
                    let token_type = if self.check_next('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                    return Ok(());
                }

                '<' => {
                    let token_type = if self.check_next('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(token_type);
                    return Ok(());
                }

                '>' => {
                    let token_type = if self.check_next('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(token_type);
                    return Ok(());
                }

                '/' => {
                    if self.check_next('/') {
                        // is a comment
                        while self.peek().is_some_and(|c| c != '\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }

                '"' => {
                    self.handle_string()?;
                    return Ok(());
                }

                _ if c.is_ascii_digit() => {
                    self.handle_number();
                    return Ok(());
                }

                _ if c.is_ascii_alphabetic() || c == '_' => {
                    self.handle_identifier();
                    return Ok(());
                }

                _ if c.is_whitespace() => {}

                _ => {
                    let error = CompileError {
                        line: self.line,
                        at: "".to_owned(),
                        message: "Unexpected character.".to_owned(),
                    };
                    return Err(LoxError::Compile(error));
                }
            }
        }
        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme, self.line);
        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.current += character.len_utf8();

        if character == '\n' {
            self.line += 1;
        }

        Some(character)
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.source[self.current..].chars().nth(1)
    }

    fn check_next(&mut self, desired: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(actual) = self.peek() {
            if actual == desired {
                self.advance();
                return true;
            }

            return false;
        } else {
            return false;
        };
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn handle_string(&mut self) -> Result<(), LoxError> {
        while self.peek().is_some_and(|c| c != '"' && !self.is_at_end()) {
            self.advance();
        }

        if self.is_at_end() {
            let error = CompileError {
                line: self.line,
                at: "".to_owned(),
                message: "Unterminated string.".to_owned(),
            };
            return Err(Compile(error));
        }

        // the closing "
        self.advance();

        let lexeme = &self.source[self.start..self.current];
        let value = &self.source[self.start + 1..self.current - 1];
        let token = Token::new(TokenType::String, lexeme, self.line)
            .with_literal(TokenLiteral::String(value.to_owned()));
        self.tokens.push(token);

        Ok(())
    }

    fn handle_number(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        // look for a fractional part
        if self.peek().is_some_and(|c| c == '.')
            && self.peek_next().is_some_and(|c| c.is_ascii_digit())
        {
            self.advance();
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        let lexeme = &self.source[self.start..self.current];
        let literal = lexeme
            .parse::<f64>()
            .expect("scanner produced an invalid number");
        let token = Token::new(TokenType::Number, lexeme, self.line)
            .with_literal(TokenLiteral::Number(literal));
        self.tokens.push(token);
    }

    fn handle_identifier(&mut self) {
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];

        let token_type = match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(source: &str) -> Vec<Token> {
        Scanner::new(source).scan().expect("source should scan")
    }

    fn token(token_type: TokenType, lexeme: &str, line: usize) -> Token {
        Token::new(token_type, lexeme, line)
    }

    fn token_with_literal(
        token_type: TokenType,
        lexeme: &str,
        literal: TokenLiteral,
        line: usize,
    ) -> Token {
        Token::new(token_type, lexeme, line).with_literal(literal)
    }

    #[test]
    fn empty_input_produces_only_eof() {
        assert_eq!(scan(""), vec![token(TokenType::Eof, "", 1)]);
    }

    #[test]
    fn scans_every_single_character_token() {
        let tokens = scan("(){}.,-+;/*");

        assert_eq!(
            tokens,
            vec![
                token(TokenType::LeftParen, "(", 1),
                token(TokenType::RightParen, ")", 1),
                token(TokenType::LeftBrace, "{", 1),
                token(TokenType::RightBrace, "}", 1),
                token(TokenType::Dot, ".", 1),
                token(TokenType::Comma, ",", 1),
                token(TokenType::Minus, "-", 1),
                token(TokenType::Plus, "+", 1),
                token(TokenType::Semicolon, ";", 1),
                token(TokenType::Slash, "/", 1),
                token(TokenType::Star, "*", 1),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn distinguishes_one_and_two_character_operators() {
        let tokens = scan("! != = == > >= < <=");

        assert_eq!(
            tokens,
            vec![
                token(TokenType::Bang, "!", 1),
                token(TokenType::BangEqual, "!=", 1),
                token(TokenType::Equal, "=", 1),
                token(TokenType::EqualEqual, "==", 1),
                token(TokenType::Greater, ">", 1),
                token(TokenType::GreaterEqual, ">=", 1),
                token(TokenType::Less, "<", 1),
                token(TokenType::LessEqual, "<=", 1),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn ignores_whitespace() {
        assert_eq!(
            scan(" \t\r\n("),
            vec![
                token(TokenType::LeftParen, "(", 2),
                token(TokenType::Eof, "", 2),
            ]
        );
    }

    #[test]
    fn tracks_token_line_numbers() {
        assert_eq!(
            scan("var\nname\n= 1;"),
            vec![
                token(TokenType::Var, "var", 1),
                token(TokenType::Identifier, "name", 2),
                token(TokenType::Equal, "=", 3),
                token_with_literal(TokenType::Number, "1", TokenLiteral::Number(1.0), 3,),
                token(TokenType::Semicolon, ";", 3),
                token(TokenType::Eof, "", 3),
            ]
        );
    }

    #[test]
    fn ignores_line_comments() {
        assert_eq!(
            scan("// first\nvar // second\nprint"),
            vec![
                token(TokenType::Var, "var", 2),
                token(TokenType::Print, "print", 3),
                token(TokenType::Eof, "", 3),
            ]
        );
    }

    #[test]
    fn ignores_a_line_comment_at_eof() {
        assert_eq!(
            scan("var // no final newline"),
            vec![
                token(TokenType::Var, "var", 1),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn scans_slash_when_it_does_not_start_a_comment() {
        assert_eq!(
            scan("/"),
            vec![
                token(TokenType::Slash, "/", 1),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn scans_integer_and_fractional_numbers() {
        assert_eq!(
            scan("0 123 45.67"),
            vec![
                token_with_literal(TokenType::Number, "0", TokenLiteral::Number(0.0), 1,),
                token_with_literal(TokenType::Number, "123", TokenLiteral::Number(123.0), 1,),
                token_with_literal(TokenType::Number, "45.67", TokenLiteral::Number(45.67), 1,),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn only_includes_a_decimal_point_when_followed_by_a_digit() {
        assert_eq!(
            scan("123. 1.2.3"),
            vec![
                token_with_literal(TokenType::Number, "123", TokenLiteral::Number(123.0), 1,),
                token(TokenType::Dot, ".", 1),
                token_with_literal(TokenType::Number, "1.2", TokenLiteral::Number(1.2), 1,),
                token(TokenType::Dot, ".", 1),
                token_with_literal(TokenType::Number, "3", TokenLiteral::Number(3.0), 1,),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn scans_identifiers_with_digits_and_underscores() {
        assert_eq!(
            scan("name thing2 foo_bar _private"),
            vec![
                token(TokenType::Identifier, "name", 1),
                token(TokenType::Identifier, "thing2", 1),
                token(TokenType::Identifier, "foo_bar", 1),
                token(TokenType::Identifier, "_private", 1),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn recognizes_every_keyword() {
        assert_eq!(
            scan("and class else false for fun if nil or print return super this true var while"),
            vec![
                token(TokenType::And, "and", 1),
                token(TokenType::Class, "class", 1),
                token(TokenType::Else, "else", 1),
                token(TokenType::False, "false", 1),
                token(TokenType::For, "for", 1),
                token(TokenType::Fun, "fun", 1),
                token(TokenType::If, "if", 1),
                token(TokenType::Nil, "nil", 1),
                token(TokenType::Or, "or", 1),
                token(TokenType::Print, "print", 1),
                token(TokenType::Return, "return", 1),
                token(TokenType::Super, "super", 1),
                token(TokenType::This, "this", 1),
                token(TokenType::True, "true", 1),
                token(TokenType::Var, "var", 1),
                token(TokenType::While, "while", 1),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn keyword_prefixes_and_suffixes_are_identifiers() {
        assert_eq!(
            scan("andy classy falsehood variable"),
            vec![
                token(TokenType::Identifier, "andy", 1),
                token(TokenType::Identifier, "classy", 1),
                token(TokenType::Identifier, "falsehood", 1),
                token(TokenType::Identifier, "variable", 1),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn string_lexeme_keeps_quotes_while_literal_does_not() {
        assert_eq!(
            scan("\"hello world\""),
            vec![
                token_with_literal(
                    TokenType::String,
                    "\"hello world\"",
                    TokenLiteral::String("hello world".to_owned()),
                    1,
                ),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn scans_an_empty_string() {
        assert_eq!(
            scan("\"\""),
            vec![
                token_with_literal(
                    TokenType::String,
                    "\"\"",
                    TokenLiteral::String(String::new()),
                    1,
                ),
                token(TokenType::Eof, "", 1),
            ]
        );
    }

    #[test]
    fn scans_a_multiline_string_and_tracks_its_ending_line() {
        assert_eq!(
            scan("\"first\nsecond\""),
            vec![
                token_with_literal(
                    TokenType::String,
                    "\"first\nsecond\"",
                    TokenLiteral::String("first\nsecond".to_owned()),
                    2,
                ),
                token(TokenType::Eof, "", 2),
            ]
        );
    }

    #[test]
    fn rejects_an_unexpected_character_on_the_correct_line() {
        let error = Scanner::new("\n@").scan().expect_err("scan should fail");

        match error {
            LoxError::Compile(error) => {
                assert_eq!(error.line, 2);
                assert_eq!(error.at, "");
                assert_eq!(error.message, "Unexpected character.");
            }
            other => panic!("expected compile error, got {other:?}"),
        }
    }

    #[test]
    fn rejects_non_ascii_identifier_characters() {
        let error = Scanner::new("café").scan().expect_err("scan should fail");

        match error {
            LoxError::Compile(error) => {
                assert_eq!(error.line, 1);
                assert_eq!(error.message, "Unexpected character.");
            }
            other => panic!("expected compile error, got {other:?}"),
        }
    }

    #[test]
    fn reports_an_unterminated_string_on_its_ending_line() {
        let error = Scanner::new("\"first\nsecond")
            .scan()
            .expect_err("scan should fail");

        match error {
            LoxError::Compile(error) => {
                assert_eq!(error.line, 2);
                assert_eq!(error.at, "");
                assert_eq!(error.message, "Unterminated string.");
            }
            other => panic!("expected compile error, got {other:?}"),
        }
    }

    #[test]
    fn successful_scan_appends_exactly_one_eof_token() {
        let tokens = scan("var value = 1;");

        assert_eq!(
            tokens
                .iter()
                .filter(|token| token.token_type == TokenType::Eof)
                .count(),
            1
        );
        assert_eq!(tokens.last(), Some(&token(TokenType::Eof, "", 1)));
    }
}
