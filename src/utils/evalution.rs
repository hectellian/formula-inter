//! A module for the evaluation of the language

use super::tokens::Token;
use super::tokenizer::Tokenizer;
use std::borrow::Borrow;
use std::mem::discriminant;

#[derive(Debug)]
enum RuntimeError {

    IncorrectType,

    PostfixingError,

    OutOfMemory

}


fn mul(a:Token,b:Token)->Result<Token,RuntimeError>{
    match a {
        Token::Real(va, ..) => {
            match b {
                Token::Real(vb,..) => return Ok(Token::Real(va*vb,0,0)),
                Token::Integer(vb,..) => return Ok(Token::Real(vb as f64 * va,0,0)),
                _ => return Err(RuntimeError::IncorrectType)
            }
        }
        Token::Integer(va,..) => {
            match b {
                Token::Real(vb,..) => return Ok(Token::Real(va as f64 * vb,0,0)),
                Token::Integer(vb,..) => Ok(Token::Integer(va*vb,0,0)),
                _ => Err(RuntimeError::IncorrectType)
            }
        }
        _ => return Err(RuntimeError::IncorrectType)
    }
}

fn add(a:Token,b:Token)->Result<Token,RuntimeError>{
    match a {
        Token::Real(va, ..) => {
            match b {
                Token::Real(vb,..) => return Ok(Token::Real(va+vb,0,0)),
                Token::Integer(vb,..) => return Ok(Token::Real(vb as f64 + va,0,0)),
                _ => return Err(RuntimeError::IncorrectType)
            }
        }
        Token::Integer(va,..) => {
            match b {
                Token::Real(vb,..) => return Ok(Token::Real(va as f64 + vb,0,0)),
                Token::Integer(vb,..) => Ok(Token::Integer(va+vb,0,0)),
                _ => Err(RuntimeError::IncorrectType)
            }
        }
        _ => return Err(RuntimeError::IncorrectType)
    }
}

fn inv(a:Token) -> Result<Token,RuntimeError>{
    match a {
        Token::Real(va,..) => Ok(Token::Real(1.0/va, 0, 0)),
        Token::Integer(va,..) => Ok(Token::Integer(1/va, 0, 0)),

        _ => return Err(RuntimeError::IncorrectType)
    }
}

fn afficher(a:Token) -> Option<RuntimeError> {
    match a {
        Token::Real(v,..) => print!("{} ",v),
        Token::Integer(v,..) => print!("{} ",v),
        _ => return Some(RuntimeError::IncorrectType)
    }

    return None;
}

fn postfix(tokens:Vec<Token>) -> Result<Vec<Token>,RuntimeError> {
    
    let mut pile:Vec<Token> = Vec::new();
    let mut openpar_num = 0;
    let mut closepar_num = 0;
    let mut res:Vec<Token> = Vec::new();

    match res.try_reserve_exact(tokens.len()).err() {
        Some(..) => {return Err(RuntimeError::OutOfMemory)},
        None => {}
    }

    for tok in tokens {
        match tok {
            Token::OpenParenthesis(..) => {
                openpar_num+=1;
                if openpar_num != 1 {
                    pile.push(tok);
                }
            },
            Token::CloseParenthesis(..) => {
                closepar_num+=1;
                if closepar_num < openpar_num{
                    pile.push(tok)
                } else if closepar_num == openpar_num {
                    let ir = postfix(pile);
                    pile = Vec::new();
                    match ir {
                        Err(e) => {return Err(e);},
                        Ok(mut v) => {
                            res.append(&mut v);
                        }
                    }
                }
            }
            _ => return Err(RuntimeError::PostfixingError)
        }
    }
    Ok(res)
}

type Var = (String,Token);

pub fn eval_expr(expr:Vec<Token>,vars:Vec<Var>) -> Token {


    Token::Integer(0,0,0)
}


pub fn evaluation(input:String) -> bool {

    let mut declared_var:Vec<Var> = Vec::new();

    let tok_stream = Tokenizer::from(input.clone());

    for tok in tok_stream {
        match tok {
            Token::Identifier(s, e, l, c) => {
                
            }
            _ => return false
        }
    }

    true
}