use crate::ast::{Expression, LiteralValue, Statement};
use crate::error::ParserError;
use crate::lexer::{Token, TokenType};

/// Recursive descent parser for the Lox language.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Option<Vec<Token>>) -> Self {
        Parser {
            tokens: tokens.unwrap_or_default(),
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn get_errors(&self) -> &Vec<ParserError> {
        &self.errors
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }
}

impl Parser {
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().unwrap()
    }

    fn previous(&mut self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn error(&mut self, message: &str) {
        self.errors.push(ParserError::UnexpectedToken {
            line: self.peek().line(),
            token: self.peek().clone(),
            msg: message.to_string(),
        });
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type() == token_type
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParserError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParserError::UnexpectedToken {
                line: self.peek().line(),
                token: self.peek().clone(),
                msg: message.to_string(),
            })
        }
    }

    // ------------ Parsing Rules ------------
    fn primary(&mut self) -> Result<Expression, ParserError> {
        let token = self.advance();
        match token.token_type() {
            TokenType::Number => Ok(Expression::Literal {
                value: LiteralValue::Number(token.lexeme().parse::<f64>().unwrap()),
            }),
            TokenType::String => Ok(Expression::Literal {
                value: LiteralValue::String(token.lexeme().to_string()),
            }),
            TokenType::False => Ok(Expression::Literal {
                value: LiteralValue::Bool(false),
            }),
            TokenType::True => Ok(Expression::Literal {
                value: LiteralValue::Bool(true),
            }),
            TokenType::Nil => Ok(Expression::Literal {
                value: LiteralValue::Nil,
            }),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                let _ = self.consume(TokenType::RightParen, "Expect ')' after expression.");
                Ok(Expression::Grouped(Box::new(expr)))
            }
            _ => Err(ParserError::ExpectedExpression {
                line: token.line(),
                token: token.clone(),
                msg: "Expect expression.".to_string(),
            }),
        }
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.match_token(&[TokenType::Minus, TokenType::Bang]) {
            Ok(Expression::Unary {
                operator: self.previous().unwrap().clone(),
                right: Box::new(self.unary()?),
            })
        } else {
            self.primary()
        }
    }

    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: self.previous().unwrap().clone(),
                right: Box::new(self.factor()?),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: self.previous().unwrap().clone(),
                right: Box::new(self.unary()?),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.term()?;
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: self.previous().unwrap().clone(),
                right: Box::new(self.term()?),
            };
        }
        Ok(expr)
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().unwrap().token_type() == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        // Parsing logic for equality expressions
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: self.previous().unwrap().clone(),
                right: Box::new(self.comparison()?),
            };
        }
        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        // Parsing logic for expressions
        let expr = self.equality()?;
        Ok(expr)
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        // Parsing logic for statements
        match self.peek().token_type() {
            TokenType::Print => self.print_statement(),
            // TokenType::LeftBrace => self.block_statement(),
            // TokenType::If => self.if_statement(),
            // TokenType::While => self.while_statement(),
            // TokenType::For => self.for_statement(),
            // TokenType::Return => self.return_statement(),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance();
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Statement::Expr(expr))
    }
}
