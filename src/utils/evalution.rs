//! A module for the evaluation of the language

use super::tokens::Token;
use super::tokenizer::Tokenizer;
use std::mem::discriminant;

#[derive(Debug)]
enum RuntimeError {

    IncorrectType,

    //PostfixingError,

    OutOfMemory,

    EvaluationIncoherence(u32),

    //UseOfUndeclaredVariable,

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
    let mut rec_pile:Vec<Token> = Vec::new();
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
                    rec_pile.push(tok);
                }
            },
            Token::CloseParenthesis(..) => {
                closepar_num+=1;
                if closepar_num < openpar_num{
                    rec_pile.push(tok);
                } else if closepar_num == openpar_num {
                    let ir = postfix(rec_pile);
                    rec_pile = Vec::new();
                    match ir {
                        Err(e) => {return Err(e);},
                        Ok(v) => {
                            for tok in v {
                                match tok {
                                    Token::Real(..)|Token::Integer(..) => {
                                        res.push(tok);
                                    },
                                    Token::Multiplier(..) => {
                                        match pile.last() {
                                            None => pile.push(tok),
                                            Some(s) => {
                                                if discriminant(s) == discriminant(&tok) {
                                                    res.push(pile.pop().unwrap());   
                                                }
                                                pile.push(tok);
                                            }
                                        }
                                    },
                                    Token::Adder(..) => {
                                        match pile.last() {
                                            None => {},
                                            Some(_s) => {
                                                res.push(pile.pop().unwrap())
                                            }
                                        }
                                        pile.push(tok);
                                    },
                                    _ => return Err(RuntimeError::IncorrectType)
                                }
                            }
                        }
                    }
                    closepar_num = 0;
                    openpar_num = 0;
                }
            },
            Token::Adder(..) => {
                match pile.last() {
                    None => {},
                    Some(..) => {
                        res.push(pile.pop().unwrap())
                    }
                }
                pile.push(tok);
            },
            Token::Multiplier(..) => {
                match pile.last() {
                    None => pile.push(tok),
                    Some(s) => {
                        if discriminant(s) == discriminant(&tok) {
                            res.push(pile.pop().unwrap());   
                        }
                        pile.push(tok);
                    }
                }
            },            
            _ => res.push(tok)
        }
    }

    while let Some(tok) = pile.pop() {
        res.push(tok);
    }
    Ok(res)
}

fn eval_expr(expr:Vec<Token>) -> Result<Token,RuntimeError> {

    match postfix(expr) {
        Err(e) => return Err(e),
        Ok(postfixed) => {
            let mut pile:Vec<Token> = Vec::new();
            for tok in postfixed {
                match tok {
                    Token::Real(..)|Token::Integer(..) => pile.push(tok),
                    Token::Multiplier(..) => {
                        match pile.pop() {
                            None => return Err(RuntimeError::EvaluationIncoherence(line!())),
                            Some(val_right) => {
                                match pile.pop() {
                                    None => return Err(RuntimeError::EvaluationIncoherence(line!())),
                                    Some(val_left) => {
                                        match mul(val_left,val_right) {
                                            Ok(r) => pile.push(r),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Token::Adder(..) => {
                        match pile.pop() {
                            None => return Err(RuntimeError::EvaluationIncoherence(line!())),
                            Some(val_right) => {
                                match pile.pop() {
                                    None => return Err(RuntimeError::EvaluationIncoherence(line!())),
                                    Some(val_left) => {
                                        match add(val_left,val_right) {
                                            Ok(r) => pile.push(r),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => return Err(RuntimeError::IncorrectType),
                }
            }
            match pile.pop() {
                None => return Err(RuntimeError::EvaluationIncoherence(line!())),
                Some(r) => return Ok(r),
            }
        }
    }
}

pub fn evaluation(input:String) -> bool {

    let mut var_name:Vec<&str> = Vec::new();
    let mut var_val:Vec<Token> = Vec::new();

    let mut pile:Vec<Token> = Vec::new();

    for tok in Tokenizer::from(input.clone()) {
        match tok {
            Token::Semicolon(..) => {
                let mut replaced_pile:Vec<Token> = Vec::new();
                replaced_pile.reserve_exact(pile.len());

                while let Some(tok) = pile.pop() {
                    match tok {
                        Token::Identifier(s, e,l,c) => {
                            let id = input.get(s..e).unwrap();
                            let mut found = false;
                            for i in 0..var_name.len() {
                                if var_name[i] == id {
                                    replaced_pile.push(var_val[i]);
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                println!("Semantical Error: use of undeclared value: '{}' at line: {} column: {}",id,l+1,c+1);
                                return false;
                            }
                        },
                        Token::Equal(..) => {
                            if let Some(tok) = pile.pop() {
                                match tok {
                                    Token::Identifier(s,e,..) => {
                                        let id = input.get(s..e).unwrap();
                                        let mut found = false;
                                        for i in 0..(var_name.len()) {
                                            if var_name[i] == id {
                                                replaced_pile.reverse();
                                                match eval_expr(replaced_pile){
                                                    Ok(r) => {
                                                        var_val[i] = r;
                                                        replaced_pile = Vec::new();
                                                        found = true;
                                                        break;},
                                                    Err(e) => {println!("{:?}",e); return false},
                                                }
                                            }
                                        }
                                        if !found {
                                            var_name.push(id);
                                            replaced_pile.reverse();
                                            match eval_expr(replaced_pile){
                                                Ok(r) => var_val.push(r),
                                                Err(e) => {println!("{:?}",e); return false},
                                            }
                                        }
                                        break;
                                    },
                                    _ => {println!("Not supposed to happen found something else than identifier in assignation")}
                                }
                            }
                        },
                        Token::AffRal(..)=>{
                            print!("\n");
                        },
                        Token::Afficher(..) => {
                            replaced_pile.reverse();
                            match eval_expr(replaced_pile) {
                                Ok(r) => {afficher(r);break;},
                                Err(e) => {println!("{:?}",e);return false;}
                            }
                        },
                        Token::Inv(..) => {
                            if let Some(tok) = pile.pop() {
                                match tok {
                                    Token::Equal(..) =>{},
                                    _ => {println!("Not supposed to happen found something else than equal before an inv token")}
                                }
                            }
                            if let Some(tok) = pile.pop() {
                                match tok {
                                    Token::Identifier(s,e,..) => {
                                        let id = input.get(s..e).unwrap();
                                        let mut found = false;
                                        for i in 0..var_name.len() {
                                            if var_name[i]==id {
                                                found = true;
                                                replaced_pile.reverse();
                                                match eval_expr(replaced_pile){
                                                    Ok(r) => {
                                                        match inv(r) {
                                                            Ok(r) => {
                                                                var_val[i] = r;
                                                                replaced_pile = Vec::new();
                                                                break;
                                                            },
                                                            Err(e) => {println!("{:?}",e); return false},        
                                                        }
                                                    },
                                                    Err(e) => {println!("{:?}",e); return false},
                                                }
                                            }
                                        }
                                        if !found {
                                            var_name.push(id);
                                            replaced_pile.reverse();
                                            match eval_expr(replaced_pile){
                                                Ok(r) => {
                                                    match inv(r) {
                                                        Ok(r) => var_val.push(r),
                                                        Err(e) => {println!("{:?}",e); return false},        
                                                    }
                                                },
                                                Err(e) => {println!("{:?}",e); return false},
                                            }
                                        }
                                        break;
                                    },
                                    _ => {println!("Not supposed to happen found something else than identifier in assignation")}
                                }
                            }
                        }
                        _ => replaced_pile.push(tok),
                    }
                }
                pile = Vec::new();
            },
            _ => pile.push(tok),
        }
    }

    true
}