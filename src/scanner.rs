use crate::{lox::{CompileError, LoxError::{self, Compile}}, tokens::{Token, TokenLiteral, TokenType}};

pub struct Scanner<'src> {
    source: &'src str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
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
                '(' => { self.add_token(TokenType::LeftParen); return Ok(()) },
                ')' => { self.add_token(TokenType::RightParen); return Ok(()) },
                '{' => { self.add_token(TokenType::LeftBrace); return Ok(()) },
                '}' => { self.add_token(TokenType::RightBrace); return Ok(()) },
                ',' => { self.add_token(TokenType::Comma); return Ok(()) },
                '.' => { self.add_token(TokenType::Dot); return Ok(()) },
                '-' => { self.add_token(TokenType::Minus); return Ok(()) },
                '+' => { self.add_token(TokenType::Plus); return Ok(()) },
                ';' => { self.add_token(TokenType::Semicolon); return Ok(()) },
                '*' => { self.add_token(TokenType::Star); return Ok(()) },

                '!' => {
                    let token_type = if self.check_next('=') { TokenType::BangEqual } else { TokenType::Bang };
                    self.add_token(token_type);
                    return Ok(());
                }

                '=' => {
                    let token_type = if self.check_next('=') { TokenType::EqualEqual } else { TokenType::Equal };
                    self.add_token(token_type);
                    return Ok(());
                }

                '<' => {
                    let token_type = if self.check_next('=') { TokenType::LessEqual } else { TokenType::Less };
                    self.add_token(token_type);
                    return Ok(());
                }

                '>' => {
                    let token_type = if self.check_next('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                    self.add_token(token_type);
                    return Ok(());
                }

                '/' => {
                    if self.check_next('/') {
                        // is a comment
                        while self.peek().is_some_and(|c| c != '\n') && !self.is_at_end() { self.advance(); }
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
                    return Ok(())
                }

                _ if c.is_alphanumeric() => {
                    self.handle_identifier();
                    return Ok(());
                }

                _ if c.is_whitespace() => { }

                _ => { 
                    let error = CompileError {
                        line: self.line, at: "".to_owned(), message: "Unexpected character.".to_owned()
                    };
                    return Err(LoxError::Compile(error)) 
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
        if self.is_at_end() { return false; }

        if let Some(actual) = self.peek() {
            if actual == desired {
                self.advance();
                return true;
            }

            return false;
        } else { return false; };
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn handle_string(&mut self) -> Result<(), LoxError> {
        while self.peek().is_some_and(|c| c != '"' && !self.is_at_end()) {
            self.advance();
        }

        if self.is_at_end() {
            let error = CompileError { line: self.line, at: "".to_owned(), message: "Unterminated string.".to_owned() };
            return Err(Compile(error));
        }

        // the closing "
        self.advance();

        // trim surrounding quotes
        let lexeme = &self.source[self.start+1..self.current-1];
        let token = Token::new(TokenType::String, lexeme, self.line).with_literal(TokenLiteral::String(lexeme.to_owned()));
        self.tokens.push(token);

        Ok(())
    }

    fn handle_number(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) { self.advance(); }

        // look for a fractional part
        if self.peek().is_some_and(|c| c == '.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
            while self.peek().is_some_and(|c| c.is_ascii_digit()) { self.advance(); }
        }

        let lexeme = &self.source[self.start..self.current];
        let literal = lexeme.parse::<f64>().expect("scanner produced an invalid number");
        let token = Token::new(TokenType::Number, lexeme, self.line).with_literal(TokenLiteral::Number(literal));
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
