use crate::error::LexError;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    QuestionMark,
    Colon,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Comments.
    LineComment,
    BlockComment,

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current_index: usize,
    start_index: usize,
    line_index: usize,
}

impl Lexer {
    pub fn new(source: Option<String>) -> Self {
        Lexer {
            source: source.unwrap_or_default(),
            tokens: Vec::new(),
            current_index: 0,
            start_index: 0,
            line_index: 1,
        }
    }

    pub fn tokenize_input(&self) -> Vec<Token> {
        // Split on whitespace
        self.source
            .as_str()
            .split_whitespace()
            .map(|lexeme| Token {
                token_type: TokenType::Identifier,
                lexeme: lexeme.to_string(),
                line: 1,
            })
            .collect()
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LexError> {
        while !self.reached_end() {
            self.start_index = self.current_index;
            self.scan_token()?;
        }
        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.line_index));
        Ok(self.tokens.clone())
    }

    fn advance(&mut self) -> Result<char, LexError> {
        match self.current_index >= self.source.len() {
            true => Err(LexError::UnexpectedEOF(1)),
            false => {
                self.current_index += 1;
                Ok(self.source.chars().nth(self.current_index - 1).unwrap())
            }
        }
    }

    fn advance_while(&mut self, predicate_fn: impl Fn(char) -> bool) {
        while self.peek().map_or(false, &predicate_fn) {
            self.advance().ok();
        }
    }

    // Peek at the next character without advancing the cursor
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current_index)
    }

    // Peek at the next character without advancing the cursor
    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current_index + 1)
    }

    // Has reached end of source code
    fn reached_end(&self) -> bool {
        self.current_index >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start_index..self.current_index].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, self.line_index));
    }

    // Handle string
    fn add_string_token(&mut self) -> Result<(), LexError> {
        while self.peek() != Some('"') && !self.reached_end() {
            let _ = self.advance();
        }
        if self.reached_end() {
            return Err(LexError::UnexpectedEOF(1));
        }
        let _ = self.advance();
        let lexeme = self.source[self.start_index + 1..self.current_index - 1].to_string();
        self.tokens
            .push(Token::new(TokenType::String, lexeme, self.line_index));
        Ok(())
    }

    fn add_number_token(&mut self) -> Result<(), LexError> {
        // Check for legal first digit
        while self.peek().unwrap().is_numeric() && self.peek().unwrap() != '0' {
            let _ = self.advance();
        }

        // Check decimal point
        if self.peek().unwrap() == '.' {
            let _ = self.advance();
            while self.peek().unwrap().is_numeric() {
                let _ = self.advance();
            }
        }

        let lexeme = self.source[self.start_index..self.current_index].to_string();
        self.tokens
            .push(Token::new(TokenType::Number, lexeme, self.line_index));
        Ok(())
    }

    fn add_keyword_or_identifier_token(&mut self) -> Result<(), LexError> {
        while self.peek().unwrap().is_alphanumeric() {
            let _ = self.advance();
        }

        let lexeme = self.source[self.start_index..self.current_index].to_string();
        let token_type = match lexeme.as_str() {
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
        self.tokens
            .push(Token::new(token_type, lexeme, self.line_index));
        Ok(())
    }

    // Scan a token from the source code.
    fn scan_token(&mut self) -> Result<(), LexError> {
        if self.reached_end() {
            Ok(self.add_token(TokenType::Eof))
        } else {
            let c = self.advance().unwrap();
            match c {
                // Single-character tokens
                '(' => Ok(self.add_token(TokenType::LeftParen)),
                ')' => Ok(self.add_token(TokenType::RightParen)),
                '[' => Ok(self.add_token(TokenType::LeftBracket)),
                ']' => Ok(self.add_token(TokenType::RightBracket)),
                '{' => Ok(self.add_token(TokenType::LeftBrace)),
                '}' => Ok(self.add_token(TokenType::RightBrace)),
                '-' => Ok(self.add_token(TokenType::Minus)),
                '+' => Ok(self.add_token(TokenType::Plus)),
                ';' => Ok(self.add_token(TokenType::Semicolon)),
                '*' => Ok(self.add_token(TokenType::Star)),

                // Complex Lexemes
                '/' => match self.peek() {
                    Some('/') => {
                        self.advance().ok();
                        self.advance_while(|c| c != '\n');
                        Ok(self.add_token(TokenType::LineComment))
                    }
                    Some('*') => {
                        self.advance().ok();
                        self.advance_while(|c| c != '*');
                        // Check for end slash since * is reserved.
                        self.advance().ok();
                        if self.peek() == Some('/') {
                            self.advance().ok();
                        }
                        Ok(self.add_token(TokenType::BlockComment))
                    }
                    _ => Ok(self.add_token(TokenType::Slash)),
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.advance().ok();
                        Ok(self.add_token(TokenType::BangEqual))
                    }
                    _ => Ok(self.add_token(TokenType::Bang)),
                },
                '=' => match self.peek() {
                    Some('=') => {
                        self.advance().ok();
                        Ok(self.add_token(TokenType::EqualEqual))
                    }
                    _ => Ok(self.add_token(TokenType::Equal)),
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.advance().ok();
                        Ok(self.add_token(TokenType::LessEqual))
                    }
                    _ => Ok(self.add_token(TokenType::Less)),
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.advance().ok();
                        Ok(self.add_token(TokenType::GreaterEqual))
                    }
                    _ => Ok(self.add_token(TokenType::Greater)),
                },

                // Strings
                '"' => self.add_string_token(),

                // Numbers
                '0'..='9' => self.add_number_token(),

                'a'..='z' | 'A'..='Z' | '_' => self.add_keyword_or_identifier_token(),

                // Ignored Lexemes
                ' ' => Ok(()),
                '\t' => Ok(()),
                '\r' => Ok(()),
                '\n' => {
                    self.line_index += 1;
                    Ok(())
                }
                _ => Err(LexError::UnexpectedEOF(1)),
            }
        }
    }
}
