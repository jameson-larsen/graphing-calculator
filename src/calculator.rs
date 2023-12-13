use std::f64::consts::{PI, E};

use crate::{scanner::TokenType, parser::ASTNodeType};

pub struct Calculator {
    instructions: Vec<CalculatorInstruction>,
    stack: Vec<f64>,
    delta: f64
}

impl Calculator {
    fn new(v: Vec<CalculatorInstruction>, delta: f64) -> Calculator {
        Calculator { instructions: v, stack: Vec::new(), delta }
    }

    //function to calculate f(x) for the expression represented by this Calculator for a given x - returns None if f is undefined at x
    pub fn calculate(&mut self, x: f64) -> Option<f64> {
        for instruction in self.instructions.iter() {
            match instruction {
                CalculatorInstruction::Push(val) => {
                    match val {
                        CalculatorValue::Num(num) => self.stack.push(*num),
                        CalculatorValue::Variable(_) => self.stack.push(x)
                    }
                },
                CalculatorInstruction::Add => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left + right);
                },
                CalculatorInstruction::Sub => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left - right);
                },
                CalculatorInstruction::Mul => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left * right);
                },
                CalculatorInstruction::Div => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    //return None on division by zero
                    if right.abs() <= self.delta { return None; }
                    self.stack.push(left / right);
                },
                CalculatorInstruction::Exp => {
                    let power = self.stack.pop().unwrap();
                    let base = self.stack.pop().unwrap();
                    let val = base.powf(power);
                    if val.is_nan() { return None; }
                    self.stack.push(val);
                },
                CalculatorInstruction::ApplyFunc(f) => {
                    let arg = self.stack.pop().unwrap();
                    if let TokenType::FunctionName(function_name) = f { 
                        match function_name.as_str() {
                            "sin" => self.stack.push(arg.sin()),
                            "cos" => self.stack.push(arg.cos()),
                            "tan" => {
                                if arg.cos().abs() <= self.delta { return None };
                                self.stack.push(arg.tan());
                            },
                            "log" => {
                                let val = arg.log10();
                                if val == std::f64::NEG_INFINITY { return None; }
                                self.stack.push(val);
                            },
                            "ln" => {
                                let val = arg.ln();
                                if val == std::f64::NEG_INFINITY { return None; }
                                self.stack.push(val);
                            },
                            "sqrt" => {
                                if arg < 0.0 { return None; }
                                self.stack.push(arg.sqrt()); 
                            }
                            "abs" => {
                                self.stack.push(arg.abs());
                            }
                            _ => ()
                        }
                    }
                }
            }
        }
        Some(self.stack.pop().unwrap())
    }
}

//function to take an AST node and produce a vector of instructions for the Calculator to run
fn generate_instructions(expression: ASTNodeType) -> Vec<CalculatorInstruction> {
    let mut instructions : Vec<CalculatorInstruction> = Vec::new();
    match expression {
        ASTNodeType::BinaryExpression(operator,left , right) => {
            instructions.append(&mut generate_instructions(*left));
            instructions.append(&mut generate_instructions(*right));
            match operator {
                TokenType::Add => instructions.push(CalculatorInstruction::Add),
                TokenType::Sub => instructions.push(CalculatorInstruction::Sub),
                TokenType::Mul => instructions.push(CalculatorInstruction::Mul),
                TokenType::Div => instructions.push(CalculatorInstruction::Div),
                TokenType::Exp => instructions.push(CalculatorInstruction::Exp),
                _ => ()
            };
        }
        ASTNodeType::UnaryExpression(operator,expression ) => {
            instructions.push(CalculatorInstruction::Push(CalculatorValue::Num(-1.0)));
            instructions.append(&mut generate_instructions(*expression));
            match operator {
                TokenType::Sub => instructions.push(CalculatorInstruction::Mul),
                _ => ()
            };
        }
        ASTNodeType::FunctionExpression(function_name, argument) => {
            instructions.append(&mut generate_instructions(*argument));
            instructions.push(CalculatorInstruction::ApplyFunc(function_name));
        }
        ASTNodeType::AtomicExpression(expression) => {
            match expression {
                TokenType::NumLiteral(num) => instructions.push(CalculatorInstruction::Push(CalculatorValue::Num(num))),
                TokenType::Variable(_) => instructions.push(CalculatorInstruction::Push(CalculatorValue::Variable(expression))),
                TokenType::Constant(c) => {
                    match c.as_str() {
                        "pi" => instructions.push(CalculatorInstruction::Push(CalculatorValue::Num(PI))),
                        "e" => instructions.push(CalculatorInstruction::Push(CalculatorValue::Num(E))),
                        _ => ()
                    }
                }
                _ => ()
            }
        }
    }
    instructions
}

//function to create a new calculator and generate instructions for it given an AST and a delta
pub fn generate_calculator(expression: ASTNodeType, delta: f64) -> Calculator {
    let instructions = generate_instructions(expression);
    Calculator::new(instructions, delta)
}
 
enum CalculatorInstruction {
    Push(CalculatorValue),
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    ApplyFunc(TokenType)
}

enum CalculatorValue {
    Variable(TokenType),
    Num(f64)
}


