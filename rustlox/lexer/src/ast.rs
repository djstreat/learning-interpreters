use crate::error::ParseError;
use crate::lexer::{Token, TokenType};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Number(num) => write!(f, "{}", num),
            LiteralValue::String(str) => write!(f, "\"{}\"", str),
            LiteralValue::Bool(bool) => write!(f, "{}", bool),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expression {
    Literal {
        value: LiteralValue,
    },
    Grouped(Box<Expression>),
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable(String),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal { value } => write!(f, "{}", value),
            Expression::Grouped(expr) => write!(f, "({})", expr),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expression::Unary { operator, right } => {
                write!(f, "({} {})", operator, right)
            }
            Expression::Variable(name) => write!(f, "{}", name),
        }
    }
}

impl Expression {
    pub fn evaluate(&self) -> Result<LiteralValue, ParseError> {
        match self {
            Expression::Literal { value } => Ok(value.clone()),
            Expression::Grouped(expr) => expr.evaluate(),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                        match operator.token_type() {
                            TokenType::Plus => Ok(LiteralValue::Number(left_num + right_num)),
                            TokenType::Minus => Ok(LiteralValue::Number(left_num - right_num)),
                            TokenType::Star => Ok(LiteralValue::Number(left_num * right_num)),
                            TokenType::Slash => Ok(LiteralValue::Number(left_num / right_num)),
                            _ => Err(ParseError::InvalidOperands(format!(
                                "{} and {}",
                                left_val, right_val
                            ))),
                        }
                    }
                    (LiteralValue::String(left_str), LiteralValue::String(right_str)) => {
                        match operator.token_type() {
                            TokenType::Plus => {
                                Ok(LiteralValue::String(left_str.clone() + right_str))
                            }
                            _ => Err(ParseError::InvalidOperands(format!(
                                "{}{}",
                                left_val, right_val
                            ))),
                        }
                    }
                    _ => Err(ParseError::InvalidOperands(format!(
                        "{} and {}",
                        left_val, right_val
                    ))),
                }
            }
            Expression::Unary { operator, right } => {
                let right_val = right.evaluate()?;
                match operator.token_type() {
                    TokenType::Minus => match right_val {
                        LiteralValue::Number(num) => Ok(LiteralValue::Number(-num)),
                        _ => Err(ParseError::InvalidOperand(format!(
                            "{}{}",
                            operator.lexeme(),
                            right
                        ))),
                    },
                    TokenType::Print => match right_val {
                        LiteralValue::String(s) => {
                            println!("{}", s);
                            Ok(LiteralValue::Nil)
                        }
                        _ => Err(ParseError::InvalidOperand(format!(
                            "{}{}",
                            operator.lexeme(),
                            right
                        ))),
                    },
                    _ => Err(ParseError::InvalidOperand(format!(
                        "{}{}",
                        operator.lexeme(),
                        right
                    ))),
                }
            }
            Expression::Variable(_) => Err(ParseError::InvalidOperand(
                "Variable evaluation not implemented".to_string(),
            )),
        }
    }
}
