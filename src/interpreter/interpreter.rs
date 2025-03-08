use crate::interpreter::environment::Environment;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::value::Value;
use crate::parser::ast::{BinaryOperator, Expression, Program, Statement};
use miette::Result;

pub struct Interpreter {
    environment: Environment,
    source: String,
}

impl Interpreter {
    pub fn new(source: String) -> Self {
        Self {
            environment: Environment::new(),
            source,
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<(), InterpreterError> {
        for statement in program.statements {
            self.execute_statement(&statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<(), InterpreterError> {
        match statement {
            Statement::Assignment {
                identifier,
                expression,
            } => {
                let value = self.evaluate_expression(expression)?;
                self.environment.define(identifier.clone(), value);
                Ok(())
            }
            Statement::Block {
                statements,
                indentation: _,
            } => {
                // Execute all statements in the block
                for stmt in statements {
                    self.execute_statement(stmt)?;
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                while self.evaluate_expression(condition)?.is_truthy() {
                    // Execute the body statement (likely a Block)
                    self.execute_statement(body)?;
                }
                Ok(())
            }
            Statement::ForLoop {
                variable,
                start_value,
                end_value,
                body,
            } => {
                let start = match self.evaluate_expression(start_value)? {
                    Value::Number(n) => n,
                    _ => {
                        return Err(InterpreterError::TypeError {
                            message: "For loop start value must be a number".to_string(),
                            src: self.source.clone(),
                            span: (0, 0).into(),
                        });
                    }
                };

                let end = match self.evaluate_expression(end_value)? {
                    Value::Number(n) => n,
                    _ => {
                        return Err(InterpreterError::TypeError {
                            message: "For loop end value must be a number".to_string(),
                            src: self.source.clone(),
                            span: (0, 0).into(),
                        });
                    }
                };

                for i in start..=end {
                    self.environment.define(variable.clone(), Value::Number(i));
                    // Execute the body statement (likely a Block)
                    self.execute_statement(body)?;
                }
                Ok(())
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // Evaluate the condition
                let condition_value = self.evaluate_expression(condition)?;

                if condition_value.is_truthy() {
                    // Execute the then branch (likely a Block)
                    self.execute_statement(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    // Execute the else branch if it exists (likely a Block)
                    self.execute_statement(else_stmt)?;
                }
                Ok(())
            }
            Statement::FunctionCall { name, arguments } => {
                match name.as_str() {
                    "print" => {
                        let mut arg_values = Vec::new();
                        for arg in arguments {
                            arg_values.push(self.evaluate_expression(arg)?);
                        }

                        // Print arguments
                        for (i, value) in arg_values.iter().enumerate() {
                            if i > 0 {
                                print!(" ");
                            }
                            print!("{}", value);
                        }
                        println!();

                        Ok(()) // Return an Ok result after printing
                    }
                    _ => {
                        Err(InterpreterError::RuntimeError {
                            message: format!("Unknown function: {}", name),
                            src: self.source.clone(),
                            span: (0, 0).into(), // Ideally we'd have a real span here
                        })
                    }
                }
            }
        }
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value, InterpreterError> {
        match expression {
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Identifier(name) => {
                if let Some(value) = self.environment.get(name) {
                    Ok(value.clone())
                } else {
                    Err(InterpreterError::UndefinedVariable {
                        name: name.clone(),
                        src: self.source.clone(),
                        span: (0, name.len()).into(), // This is a placeholder for now
                    })
                }
            }
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;

                match (operator, &left_val, &right_val) {
                    (BinaryOperator::Add, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l + r))
                    }
                    (BinaryOperator::Subtract, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l - r))
                    }
                    (BinaryOperator::Multiply, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l * r))
                    }
                    (BinaryOperator::Divide, Value::Number(l), Value::Number(r)) => {
                        if *r == 0 {
                            return Err(InterpreterError::RuntimeError {
                                message: "Division by zero".to_string(),
                                src: self.source.clone(),
                                span: (0, 0).into(), // Ideally we'd have a real span
                            });
                        }
                        Ok(Value::Number(l / r))
                    }
                    (BinaryOperator::Equals, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l == r))
                    }
                    (BinaryOperator::GreaterThan, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l > r))
                    }
                    (BinaryOperator::GreaterThanEquals, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l >= r))
                    }
                    (BinaryOperator::LessThan, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l < r))
                    }
                    (BinaryOperator::LessThanEquals, Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l <= r))
                    }
                    // Add support for string concatenation
                    (BinaryOperator::Add, Value::String(l), Value::String(r)) => {
                        Ok(Value::String(l.clone() + r))
                    }
                    (BinaryOperator::Add, Value::String(l), _) => {
                        // Convert right value to string and concatenate
                        Ok(Value::String(l.clone() + &right_val.to_string()))
                    }
                    (BinaryOperator::Add, _, Value::String(r)) => {
                        // Convert left value to string and concatenate
                        Ok(Value::String(left_val.to_string() + r))
                    }
                    _ => Err(InterpreterError::TypeError {
                        message: format!(
                            "Cannot apply operator {:?} to {:?} and {:?}",
                            operator, left_val, right_val
                        ),
                        src: self.source.clone(),
                        span: (0, 0).into(), // Placeholder
                    }),
                }
            }
            Expression::FunctionCall { name, arguments: _ } => {
                // This is a function call in expression context
                // For now, this is not supported in our simple language
                Err(InterpreterError::RuntimeError {
                    message: format!("Function calls not supported in expressions: {}", name),
                    src: self.source.clone(),
                    span: (0, 0).into(),
                })
            }
        }
    }
}
