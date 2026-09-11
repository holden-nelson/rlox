use crate::{
    ast::{BinaryOperator, Expression, Literal, UnaryOperator},
    lox::CompileError,
    tokens::{
        Token, TokenLiteral,
        TokenType::{
            self, Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less,
            LessEqual, Minus, Nil, Number, Plus, RightParen, Slash, Star, String as StringToken,
            True,
        },
    },
};

type ParseResult<T> = Result<T, CompileError>;

pub struct Parser<'tokens> {
    tokens: &'tokens Vec<Token>,
    current: usize,
}

impl<'tokens> Parser<'tokens> {
    pub fn new(tokens: &'tokens Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> ParseResult<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Expression> {
        let mut expr = self.comparison()?;

        while self.check_next(vec![BangEqual, EqualEqual]) {
            let operator = BinaryOperator::try_from(
                self.previous()
                    .expect("check_next succeeded without advancing"),
            )?;

            let right = self.comparison()?;
            expr = Expression::Binary {
                lhs: Box::new(expr),
                operator,
                rhs: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expression> {
        let mut expr = self.term()?;

        while self.check_next(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = BinaryOperator::try_from(
                self.previous()
                    .expect("check_next succeeded without advancing"),
            )?;

            let right = self.term()?;
            expr = Expression::Binary {
                lhs: Box::new(expr),
                operator,
                rhs: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expression> {
        let mut expr = self.factor()?;

        while self.check_next(vec![Minus, Plus]) {
            let operator = BinaryOperator::try_from(
                self.previous()
                    .expect("check_next succeeded without advancing"),
            )?;

            let right = self.factor()?;

            expr = Expression::Binary {
                lhs: Box::new(expr),
                operator,
                rhs: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expression> {
        let mut expr = self.unary()?;

        while self.check_next(vec![Slash, Star]) {
            let operator = BinaryOperator::try_from(
                self.previous()
                    .expect("check_next succeeded without advancing"),
            )?;

            let right = self.unary()?;

            expr = Expression::Binary {
                lhs: Box::new(expr),
                operator,
                rhs: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expression> {
        if self.check_next(vec![Bang, Minus]) {
            let operator = UnaryOperator::try_from(
                self.previous()
                    .expect("check_next succeeded without advancing"),
            )?;

            let right = self.unary()?;

            return Ok(Expression::Unary {
                operator,
                rhs: Box::new(right),
            });
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> ParseResult<Expression> {
        if self.check_next(vec![False]) {
            return Ok(Expression::Literal(Literal::False));
        }

        if self.check_next(vec![True]) {
            return Ok(Expression::Literal(Literal::True));
        }

        if self.check_next(vec![Nil]) {
            return Ok(Expression::Literal(Literal::Nil));
        }

        if self.check_next(vec![Number, StringToken]) {
            let token = self
                .previous()
                .expect("check_next succeeded without advancing");

            let literal = match &token.literal {
                Some(TokenLiteral::Number(value)) => Literal::Number(*value),
                Some(TokenLiteral::String(value)) => Literal::String(value.clone()),
                None => {
                    return Err(CompileError {
                        line: token.line,
                        at: token.lexeme.clone(),
                        message: "Expected a literal value.".to_owned(),
                    });
                }
            };

            return Ok(Expression::Literal(literal));
        }

        if self.check_next(vec![LeftParen]) {
            let expression = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.")?;
            return Ok(Expression::Grouping(Box::new(expression)));
        }

        let token = self.peek().expect("token stream should end with EOF");
        Err(CompileError {
            line: token.line,
            at: token.lexeme.clone(),
            message: "Expect expression.".to_owned(),
        })
    }

    fn consume(&mut self, desired: TokenType, message: &str) -> ParseResult<&Token> {
        if self.peek().is_some_and(|token| token.token_type == desired) {
            return Ok(self.advance().expect("peek succeeded without a token"));
        }

        let token = self.peek().expect("token stream should end with EOF");
        Err(CompileError {
            line: token.line,
            at: token.lexeme.clone(),
            message: message.to_owned(),
        })
    }

    fn check_next(&mut self, desired: Vec<TokenType>) -> bool {
        if self.is_at_end() {
            return false;
        }

        for token_type in desired {
            if self.peek().is_some_and(|t| t.token_type == token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn is_at_end(&self) -> bool {
        self.current == self.tokens.len() - 1
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens[self.current..].iter().next()
    }

    fn advance(&mut self) -> Option<&Token> {
        let i = self.current;
        self.current += 1;
        self.tokens.get(i)
    }

    fn previous(&mut self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
