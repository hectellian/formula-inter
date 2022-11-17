#![allow(dead_code,unused_variables)]

use crate::utils::lexer::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalError {
    /** Invalid or Unkown Token */
    InvalidToken,

    /** Expression is Empty */
    InvalidExpression,

    /** Division by zero */
    InvalidOperator,

    /** Invalid number of arguments */
    InvalidOperand,

    /** Identifier or Variable */
    NonNumericalValue
}

#[derive(Debug, Clone, Copy)]
pub enum EvalResult {
    /** Integer result */
    Integer(i32),

    /** Float result */
    Float(f32),

    /** Error */
    Error(EvalError)
}

fn calc(v1: f32, op: char, v2: f32) -> f32 {
    match op {
            '^' => v1.powf(v2),
            '*' => v1*v2,
            '/' => v1/v2,
            '+' => v1+v2,
            '-' => v1-v2,
            _ => 0.0,
    }
}

pub fn eval(postfixe: Vec<Token>) -> EvalResult {
    let mut stack = Vec::new();
    for token in postfixe {
        match token {
            Token::Operator{raw, kind} => {
                if stack.len() < 2 {
                    return EvalResult::Error(EvalError::InvalidExpression);
                }
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                let result = calc(v1, raw, v2);
                stack.push(result);
            },
            Token::Integer(s) => {
                let v = s.parse::<f32>().unwrap();
                stack.push(v);
            },
            Token::Real(s) => {
                let v = s.parse::<f32>().unwrap();
                stack.push(v);
            },
            Token::EOF => break,
            Token::Identifier(_) => return EvalResult::Error(EvalError::NonNumericalValue),
            _ => return EvalResult::Error(EvalError::InvalidToken)
        }
    }
    if stack.len() != 1 {
        return EvalResult::Error(EvalError::InvalidExpression);
    }
    let result = stack.pop().unwrap();
    if result.trunc() == result {
        return EvalResult::Integer(result as i32);
    }
    EvalResult::Float(result)
}