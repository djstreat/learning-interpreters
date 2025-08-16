use crate::error::RuntimeError;
use crate::interpreter::LoxValue;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            parent: None,
            variables: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            parent: Some(Box::new(parent)),
            variables: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: &str) -> Result<&LoxValue, RuntimeError> {
        match self.variables.get(name) {
            Some(value) => Ok(value),
            None => match &mut self.parent {
                Some(parent) => parent.get(name),
                None => Err(RuntimeError::UndefinedVariable {
                    line: 0, // TODO: Figure out how to get the line number
                    name: name.to_string(),
                }),
            },
        }
    }

    pub fn set(&mut self, name: &str, value: LoxValue) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &str, value: LoxValue) -> Result<(), RuntimeError> {
        match self.variables.get_mut(name) {
            Some(existing_value) => {
                *existing_value = value;
                Ok(())
            }
            None => match &mut self.parent {
                Some(parent) => parent.assign(name, value),
                None => Err(RuntimeError::UndefinedVariable {
                    line: 0, // TODO: Figure out how to get the line number
                    name: name.to_string(),
                }),
            },
        }
    }
}
