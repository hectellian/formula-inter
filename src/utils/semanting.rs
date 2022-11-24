//! Semantical Analysis Module


use super::tokens::Token;
use super::tokenizer::Tokenizer;

macro_rules! error_print {
    ($tok:expr,$func:expr) => {
        println!("Semantic Error: token {} found in a {} bloc",$tok,$func);   
    };
}

pub fn semantical_analysis(input:String){
    let tokens:Vec<Token> = Tokenizer::from(input).collect();
}

pub fn f(tokens:Vec<Token>) -> bool {
     
    let mut res = true;
    let t = tokens.first().copied();
    match t {
        Some(t) => {
            match t {
                Token::OpenParenthesis => {
                    let mut rec = vec![];
                    for t in tokens {
                        if t == Token::CloseParenthesis {
                            break;
                        }
                        rec.push(t);
                    }
                },
                Token::Integer(_) | Token::Real(_) | Token::Identifier(..) => {},
                _ => {
                    error_print!(t,"F");
                }
            }
        },
        None => {
            error_print!("not","F");
        }
    }
    res
}

fn e(tokens:Vec<Token>) {

}

fn d(tokens:Vec<Token>){

}
