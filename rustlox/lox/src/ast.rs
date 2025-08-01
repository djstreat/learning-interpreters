use crate::error::ParserError;
use crate::lexer::{Token, TokenType};
use std::fmt::Display;

pub trait ExpressionVisitor<T> {
    fn visit_literal(&mut self, value: &LiteralValue) -> T;
    fn visit_grouped(&mut self, expr: &Expression) -> T;
    fn visit_binary(&mut self, left: &Expression, operator: &Token, right: &Expression) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expression) -> T;
    fn visit_variable(&mut self, name: &str) -> T;
}

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

impl Expression {
    // Accept method for visitor pattern
    pub fn accept<T>(&self, visitor: &mut dyn ExpressionVisitor<T>) -> T {
        match self {
            Expression::Literal { value } => visitor.visit_literal(value),
            Expression::Grouped(expr) => visitor.visit_grouped(expr),
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expression::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expression::Variable(name) => visitor.visit_variable(name),
        }
    }
}

// AST Printer implementing ExpressionVisitor
pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }

    pub fn print(&mut self, expr: &Expression) -> String {
        expr.accept(self)
    }
}

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, exprs: &[&Expression]) -> String {
        let mut result = String::from("(");
        result.push_str(name);

        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self));
        }

        result.push(')');
        result
    }
}

impl ExpressionVisitor<String> for AstPrinter {
    fn visit_literal(&mut self, value: &LiteralValue) -> String {
        match value {
            LiteralValue::Number(num) => num.to_string(),
            LiteralValue::String(str) => format!("\"{}\"", str),
            LiteralValue::Bool(bool) => bool.to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    fn visit_grouped(&mut self, expr: &Expression) -> String {
        self.parenthesize("group", &[expr])
    }

    fn visit_binary(&mut self, left: &Expression, operator: &Token, right: &Expression) -> String {
        self.parenthesize(operator.lexeme(), &[left, right])
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expression) -> String {
        self.parenthesize(operator.lexeme(), &[right])
    }

    fn visit_variable(&mut self, name: &str) -> String {
        name.to_string()
    }
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
    pub fn evaluate(&self) -> Result<LiteralValue, ParserError> {
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
                            _ => Err(ParserError::InvalidOperand {
                                line: operator.line(),
                                token: operator.clone(),
                                msg: "Invalid operand for binary operator".to_string(),
                            }),
                        }
                    }
                    (LiteralValue::String(left_str), LiteralValue::String(right_str)) => {
                        match operator.token_type() {
                            TokenType::Plus => {
                                Ok(LiteralValue::String(left_str.clone() + right_str.as_str()))
                            }
                            _ => Err(ParserError::InvalidOperand {
                                line: operator.line(),
                                token: operator.clone(),
                                msg: "Invalid operand for binary operator".to_string(),
                            }),
                        }
                    }
                    _ => Err(ParserError::InvalidOperand {
                        line: operator.line(),
                        token: operator.clone(),
                        msg: "Invalid operand for binary operator".to_string(),
                    }),
                }
            }
            Expression::Unary { operator, right } => {
                let right_val = right.evaluate()?;
                match operator.token_type() {
                    TokenType::Minus => match right_val {
                        LiteralValue::Number(num) => Ok(LiteralValue::Number(-num)),
                        _ => Err(ParserError::InvalidOperand {
                            line: operator.line(),
                            token: operator.clone(),
                            msg: "Invalid operand for unary operator".to_string(),
                        }),
                    },
                    TokenType::Print => match right_val {
                        LiteralValue::String(s) => {
                            println!("{}", s);
                            Ok(LiteralValue::Nil)
                        }
                        _ => Err(ParserError::InvalidOperand {
                            line: operator.line(),
                            token: operator.clone(),
                            msg: "Invalid operand for print operator".to_string(),
                        }),
                    },
                    _ => Err(ParserError::InvalidOperand {
                        line: operator.line(),
                        token: operator.clone(),
                        msg: "Invalid operand for unary operator".to_string(),
                    }),
                }
            }
            Expression::Variable(_) => Err(ParserError::FunctionalityNotImplemented(
                "Variable functionality not implemented".to_string(),
            )),
        }
    }
}
