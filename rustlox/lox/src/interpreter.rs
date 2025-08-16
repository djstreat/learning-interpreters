use crate::ast::{
    DeclarationVisitor, Expression, ExpressionVisitor, LiteralValue, Statement, StatementVisitor,
};
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl LoxValue {
    pub fn stringify(&self) -> String {
        match self {
            LoxValue::Number(num) => {
                let num_str = num.to_string();
                // remove trailing 0's
                if num_str.ends_with(".0") {
                    num_str[..num_str.len() - 2].to_string()
                } else {
                    num_str
                }
            }
            LoxValue::String(str) => str.clone(),
            LoxValue::Bool(bool) => bool.to_string(),
            LoxValue::Nil => "nil".to_string(),
        }
    }
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }
    pub fn evaluate(&mut self, expr: &Expression) -> Result<LoxValue, RuntimeError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Statement) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        statements: &[Statement],
        env: &Environment,
    ) -> Result<(), RuntimeError> {
        let previous_env = self.env.clone();
        self.env = env.clone();
        for statement in statements {
            let res = match self.execute(statement) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            };
            match res {
                Err(err) => panic!("{}", err),
                _ => (),
            }
        }
        self.env = previous_env;
        Ok(())
    }

    pub fn interpret(&mut self, statements: &[Statement]) {
        for statement in statements {
            let res = match self.execute(statement) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            };
            match res {
                Err(err) => panic!("{}", err),
                _ => (),
            }
        }
    }
}

impl Interpreter {
    fn is_truthy(&self, value: &LoxValue) -> bool {
        match value {
            LoxValue::Bool(bool) => *bool,
            LoxValue::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, a: &LoxValue, b: &LoxValue) -> bool {
        match (a, b) {
            (LoxValue::Number(a), LoxValue::Number(b)) => a == b,
            (LoxValue::String(a), LoxValue::String(b)) => a == b,
            (LoxValue::Bool(a), LoxValue::Bool(b)) => a == b,
            (LoxValue::Nil, LoxValue::Nil) => true,
            (LoxValue::Nil, _) | (_, LoxValue::Nil) => false,
            _ => false,
        }
    }
    fn check_number_operand(
        &self,
        operator: &Token,
        operand: &LoxValue,
    ) -> Result<f64, RuntimeError> {
        match operand {
            LoxValue::Number(num) => Ok(*num),
            _ => Err(RuntimeError::TypeError {
                line: operator.line(),
                msg: "Operand must be a number.".to_string(),
            }),
        }
    }

    fn check_number_operands(
        &self,
        operator: &Token,
        left: &LoxValue,
        right: &LoxValue,
    ) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok((*l, *r)),
            _ => Err(RuntimeError::TypeError {
                line: operator.line(),
                msg: "Operands must be numbers.".to_string(),
            }),
        }
    }
}

// Implement ExpressionVisitor for Interpreter
impl ExpressionVisitor<Result<LoxValue, RuntimeError>> for Interpreter {
    fn visit_literal(&mut self, value: &LiteralValue) -> Result<LoxValue, RuntimeError> {
        // Convert literal value to LoxValue
        let lox_value = match value {
            LiteralValue::Number(num) => LoxValue::Number(*num),
            LiteralValue::String(str) => LoxValue::String(str.clone()),
            LiteralValue::Bool(bool) => LoxValue::Bool(*bool),
            LiteralValue::Nil => LoxValue::Nil,
        };
        Ok(lox_value)
    }

    fn visit_grouped(&mut self, expression: &Expression) -> Result<LoxValue, RuntimeError> {
        self.evaluate(expression)
    }

    fn visit_unary(
        &mut self,
        operator: &Token,
        right: &Expression,
    ) -> Result<LoxValue, RuntimeError> {
        let right = self.evaluate(right)?;
        match operator.token_type() {
            TokenType::Minus => match right {
                LoxValue::Number(num) => Ok(LoxValue::Number(-num)),
                _ => Err(RuntimeError::TypeError {
                    line: operator.line(),
                    msg: "Operand must be a number.".to_string(),
                }),
            },
            TokenType::Bang => match right {
                LoxValue::Bool(_) => Ok(LoxValue::Bool(!self.is_truthy(&right))),
                _ => Err(RuntimeError::TypeError {
                    line: operator.line(),
                    msg: "Operand must be a boolean.".to_string(),
                }),
            },
            _ => Err(RuntimeError::TypeError {
                line: operator.line(),
                msg: "Unexpected token.".to_string(),
            }),
        }
    }

    fn visit_binary(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Result<LoxValue, RuntimeError> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;
        match operator.token_type() {
            //Arithmetic operations
            TokenType::Plus => match (&left_value, &right_value) {
                (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l + r)),
                (LoxValue::String(l), LoxValue::String(r)) => {
                    Ok(LoxValue::String(format!("{}{}", l, r)))
                }
                _ => Err(RuntimeError::TypeError {
                    line: operator.line(),
                    msg: "Operands must be two numbers or two strings.".to_string(),
                }),
            },
            TokenType::Minus => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(LoxValue::Number(l - r))
            }
            TokenType::Star => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(LoxValue::Number(l * r))
            }
            TokenType::Slash => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(LoxValue::Number(l / r))
            }
            TokenType::Greater => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(LoxValue::Bool(l > r))
            }
            TokenType::GreaterEqual => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(LoxValue::Bool(l >= r))
            }
            TokenType::Less => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(LoxValue::Bool(l < r))
            }
            TokenType::LessEqual => {
                let (l, r) = self.check_number_operands(operator, &left_value, &right_value)?;
                Ok(LoxValue::Bool(l <= r))
            }
            // Equality operators
            TokenType::EqualEqual | TokenType::BangEqual => {
                Ok(LoxValue::Bool(self.is_equal(&left_value, &right_value)))
            }
            _ => Err(RuntimeError::TypeError {
                line: operator.line(),
                msg: "Unsupported operator.".to_string(),
            }),
        }
    }

    fn visit_variable(&mut self, name: &str) -> Result<LoxValue, RuntimeError> {
        match self.env.get(name) {
            Ok(value) => Ok(value.clone()),
            Err(err) => Err(err),
        }
    }

    fn visit_assign(&mut self, name: &str, value: &Expression) -> Result<LoxValue, RuntimeError> {
        let value = self.evaluate(value)?;
        self.env.assign(name, value.clone())?;
        Ok(value)
    }
}

impl StatementVisitor for Interpreter {
    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print(&mut self, expr: &Expression) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        println!("{}", value.stringify());
        Ok(())
    }

    fn visit_var_statement(
        &mut self,
        name: &str,
        initializer: &Expression,
    ) -> Result<(), RuntimeError> {
        let value = self.evaluate(initializer)?;
        self.env.set(name, value);
        Ok(())
    }

    fn visit_block(&mut self, statements: &[Statement]) -> Result<(), RuntimeError> {
        self.execute_block(statements, &self.env.clone())?;
        Ok(())
    }
}

impl DeclarationVisitor for Interpreter {
    fn visit_var_declaration(&mut self, statement: &Statement) -> Result<(), RuntimeError> {
        match statement {
            Statement::Expr(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Statement::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value.stringify());
                Ok(())
            }
            Statement::Variable { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => LoxValue::Nil,
                };
                self.env.set(name, value);
                Ok(())
            }
            Statement::Block(statements) => {
                self.execute_block(statements, &Environment::new())?;
                Ok(())
            }
        }
    }
}
