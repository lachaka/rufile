use std::collections::HashMap;
use std::io;

use super::operations::OperationExecutor;

#[derive(PartialEq)]
pub enum InputMode {
    Editing,
    Normal,
    Error,
}

enum OperationError {
    InvalidArgument,
    OperationNotFound,
    WrongArgumentCount,
}

type Operation = fn(&mut OperationExecutor, args: Vec<&str>) -> io::Result<()>;

pub struct CommandHandler {
    pub input: String,
    pub input_mode: InputMode,
    executor: OperationExecutor,
    operations: HashMap<char, Operation>,
}

impl Default for CommandHandler {
    fn default() -> CommandHandler {
        let mut ops: HashMap<char, Operation> = HashMap::new();
        ops.insert('c', OperationExecutor::copy);
        ops.insert('d', OperationExecutor::delete);
        ops.insert('e', OperationExecutor::edit);
        ops.insert('m', OperationExecutor::cut);
        ops.insert('n', OperationExecutor::create);
        ops.insert('p', OperationExecutor::paste);
        ops.insert('r', OperationExecutor::rename);
        
        CommandHandler {
            input: String::new(),
            input_mode: InputMode::Normal,
            executor: OperationExecutor::default(),
            operations: ops,
        }
    }
}

impl CommandHandler {
    pub fn exec(&mut self, file_name: Option<&str>) {
        let command :Vec<&str> = self.input
            .split_ascii_whitespace()
            .collect();

        let op = command[0].chars().nth(1).unwrap();

        if file_name.is_none() && op != 'n' && op != 'p' {
            self.input_mode = InputMode::Error;
            self.input.drain(..);
            return;
        } 

        match self.validate_input(&command) {
            Ok(()) => {
                let mut arguments: Vec<&str> = vec![];
                
                if op != 'n' && op != 'p' {
                    arguments.push(file_name.unwrap());
                }

                for arg in command.iter().skip(1) {
                    arguments.push(arg);
                } 

                let function = self.operations.get(&op).unwrap();
                
                match function(&mut self.executor, arguments) {
                    Ok(_) => self.input_mode = InputMode::Normal,
                    Err(_) => self.input_mode = InputMode::Error,
                }
            }
            Err(_) => {
                self.input_mode = InputMode::Error
            }
        }

        self.input.drain(..);
    }

    fn validate_permissions(&self, perms: &str) -> Result<(), OperationError> {
        if perms.len() != 3 {
            return Err(OperationError::InvalidArgument);
        }

        for p in perms.chars() {
            if p > '7' && p < '0' {
                return Err(OperationError::InvalidArgument);
            }
        }

        Ok(())
    }

    fn validate_input(&self, command: &Vec<&str>) -> Result<(), OperationError> {
        let operation = command[0].chars().nth(1).unwrap();
        if !self.operations.contains_key(&operation) {
            return Err(OperationError::OperationNotFound);
        }

        match operation {
            'c' | 'd' | 'm' | 'p' => {
                if command.len() != 1 {
                    return Err(OperationError::WrongArgumentCount);
                }
            }
            'e' | 'r' => {
                if command.len() != 2 {
                    return Err(OperationError::WrongArgumentCount);
                }

                if operation == 'e' {
                    return self.validate_permissions(command[1]);
                }
            }
            'n' => {
                if command.len() != 3 {
                    return Err(OperationError::WrongArgumentCount);
                }
                if command[1] != "d" && command[1] != "f" {
                    return Err(OperationError::InvalidArgument);
                }
            }
            _ => {
                return Err(OperationError::OperationNotFound);
            }
        }

        Ok(())
    }
}