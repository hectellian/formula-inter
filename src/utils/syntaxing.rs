//! Semantical Analysis Module


use super::tokens::Token;
use super::tokenizer::Tokenizer;

macro_rules! error_print {
    ($tok:expr,$func:expr) => {
        println!("Syntax Error: token {} found in a {} bloc",$tok,$func);   
    };
}

pub fn syntaxical_analysis(input:String){
    let tokens:Vec<Token> = Tokenizer::from(input).collect();
}

pub fn f(tokens:Vec<Token>,start:usize,end:usize) -> bool {
    match tokens[start] {
        Token::Integer(..) | Token::Real(..) | Token::Identifier(..) => {return true},
        Token::OpenParenthesis(..) => {
            if matches!(tokens[end],Token::CloseParenthesis(..)) {
                return e(tokens,start+1,end-1);
            }
            return false;
        },
        _ => {
            error_print!(tokens[start],"F");
            return false;
        }
    }
}

fn e(tokens:Vec<Token>,start:usize,end:usize) -> bool {
    true
}

fn d(tokens:Vec<Token>,start:usize,end:usize) -> bool{
    if start != end {
        match tokens[start] {
            Token::Adder(..) => {
                return e(tokens,start+1,end);
            }, 
            _ => {
                error_print!(tokens[start],"D");
                return false;
            }
        }
    }
    true
}

fn g(tokens:Vec<Token>,start:usize,end:usize) -> bool {
    if start != end {
        match tokens[start] {
            Token::Multiplier(..) => {
                return t(tokens,start+1,end);
            },
            _ => {
                error_print!(tokens[start],"g");
                return false;
            }
        }
    }
    true
}

fn t(tokens:Vec<Token>,start:usize,end:usize) -> bool {
    true
}