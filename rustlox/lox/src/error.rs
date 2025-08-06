use crate::lexer::Token;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnterminatedString(usize),
    UnexpectedCharacter(usize, char),
    UnexpectedToken(usize, String),
    UnexpectedEOF(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    ExpectedExpression {
        line: usize,
        token: Token,
        msg: String,
    },
    FunctionalityNotImplemented(String),
    InvalidOperand {
        line: usize,
        token: Token,
        msg: String,
    },
    NoPreviousToken {
        line: usize,
        msg: String,
    },
    UnexpectedToken {
        line: usize,
        token: Token,
        msg: String,
    },
    UnexpectedEOF {
        line: usize,
        msg: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    TypeError { line: usize, msg: String },
    UndefinedVariable { line: usize, name: String },
    CannotAssignToConstant { line: usize, name: String },
    CannotAssignToNumber { line: usize, name: String },
    CannotAssignToFunction { line: usize, name: String },
}

impl ParserError {
    pub fn line(&self) -> usize {
        match self {
            ParserError::ExpectedExpression { line, .. } => *line,
            ParserError::InvalidOperand { line, .. } => *line,
            ParserError::NoPreviousToken { line, .. } => *line,
            ParserError::UnexpectedToken { line, .. } => *line,
            ParserError::UnexpectedEOF { line, .. } => *line,
            ParserError::FunctionalityNotImplemented(_) => 0,
        }
    }

    pub fn message(&self) -> String {
        match self {
            ParserError::ExpectedExpression { line, token, msg } => {
                format!(
                    "Expected expression at line {}, token {:?}: {}",
                    line, token, msg
                )
            }
            ParserError::InvalidOperand { line, token, msg } => {
                format!(
                    "Invalid operand at line {}, token {:?}: {}",
                    line, token, msg
                )
            }
            ParserError::NoPreviousToken { line, msg } => {
                format!("No previous token at line {}: {}", line, msg)
            }
            ParserError::UnexpectedToken { line, token, msg } => {
                format!(
                    "Unexpected token at line {}, token {:?}: {}",
                    line, token, msg
                )
            }
            ParserError::UnexpectedEOF { line, msg } => {
                format!("Unexpected end of file at line {}: {}", line, msg)
            }
            ParserError::FunctionalityNotImplemented(_) => {
                "Functionality not implemented".to_string()
            }
        }
    }
}

impl RuntimeError {
    pub fn message(&self) -> String {
        match self {
            RuntimeError::CannotAssignToConstant { line, name } => {
                format!(
                    "ERROR at line {}: Cannot assign to constant '{}'",
                    line, name
                )
            }
            RuntimeError::CannotAssignToFunction { line, name } => {
                format!(
                    "ERROR at line {}: Cannot assign to function '{}'",
                    line, name
                )
            }
            RuntimeError::CannotAssignToNumber { line, name } => {
                format!("ERROR at line {}: Cannot assign to number '{}'", line, name)
            }
            RuntimeError::UndefinedVariable { line, name } => {
                format!("ERROR at line {}: Undefined variable '{}'", line, name)
            }
            RuntimeError::TypeError { line, msg } => {
                format!("ERROR at line {}: Type error: {}", line, msg)
            }
            _ => "Unknown runtime error".to_string(),
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
