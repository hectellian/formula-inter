//! A module for the evaluation of the language

use super::tokens::Token;
use super::tokenizer::Tokenizer;
use std::mem::discriminant;

#[derive(Debug)]
pub enum RuntimeError {

    IncorrectType,

    //PostfixingError,

    //OutOfMemory,

    EvaluationIncoherence(u32),

    UseOfUndeclaredVariable,

    IterationOverNonPositiveInteger,

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

fn sqrt(a:Token) -> Result<Token,RuntimeError>{
    match a {
        Token::Real(va,..) => Ok(Token::Real(va.sqrt(), 0, 0)),
        Token::Integer(va,..) => Ok(Token::Real((va as f64).sqrt(), 0, 0)),

        _ => return Err(RuntimeError::IncorrectType)
    }
}

fn afficher(a:Token) -> Result<(),RuntimeError> {
    match a {
        Token::Real(v,..) => print!("{} ",v),
        Token::Integer(v,..) => print!("{} ",v),
        _ => return Err(RuntimeError::IncorrectType)
    }

    Ok(())
}

fn postfix(tokens:Vec<Token>) -> Result<Vec<Token>,RuntimeError> {
    
    let mut pile:Vec<Token> = Vec::new();
    let mut rec_pile:Vec<Token> = Vec::new();
    let mut openpar_num:usize = 0;
    let mut closepar_num:usize = 0;
    let mut res:Vec<Token> = Vec::new();

    res.reserve_exact(tokens.len());

    let mut it = tokens.into_iter();

    while let Some(tok) = it.next() {
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
                    let mut v = postfix(rec_pile)?;
                    rec_pile = Vec::new();
                    res.append(&mut v);
                    closepar_num = 0;
                    openpar_num = 0;
                }
            },
            Token::Adder(..) => {
                if openpar_num != 0 {
                    rec_pile.push(tok);
                    continue;
                }
                if pile.last().is_some() {
                    res.push(pile.pop().unwrap())
                }
                pile.push(tok);
            },
            Token::Multiplier(..) => {
                if openpar_num != 0 {
                    rec_pile.push(tok);
                    continue;
                }
                if let Some(s) =  pile.last() {
                    if discriminant(s) == discriminant(&tok) {
                        res.push(pile.pop().unwrap());   
                    }
                }
                pile.push(tok);
            },            
            _ => {
                if openpar_num != 0 {
                    rec_pile.push(tok);
                    continue;
                }
                res.push(tok);
            }
        }
    }

    while let Some(tok) = pile.pop() {
        res.push(tok);
    }
    Ok(res)
}

fn eval_expr(expr:Vec<Token>) -> Result<Token,RuntimeError> {

    let mut pile:Vec<Token> = Vec::new();
    for tok in postfix(expr)? {
        match tok {
            Token::Real(..)|Token::Integer(..) => pile.push(tok),
            Token::Multiplier(..) => {
                match pile.pop() {
                    None => return Err(RuntimeError::EvaluationIncoherence(line!())),
                    Some(val_right) => {
                        match pile.pop() {
                            None => return Err(RuntimeError::EvaluationIncoherence(line!())),
                            Some(val_left) => {
                                pile.push(mul(val_left,val_right)?);
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
                                pile.push(add(val_left,val_right)?);
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

fn replace_vars(input:&String,vars:&Vec<(&str,Token)>,expr:Vec<Token>) -> Result<Vec<Token>,RuntimeError> {
    let mut replaced_expr:Vec<Token> = Vec::new();
    replaced_expr.reserve_exact(expr.len());

    for tok in expr  {
        match tok {
            Token::Identifier(s, e, _) => {
                let id = input.get(s..e).unwrap_or_default();
                if let Some(v) = vars.iter().find(|&&var|var.0==id){
                    replaced_expr.push(v.1);
                } else {
                    //let _pos = p.unwrap();
                    //println!("Semantical Error: use of undeclared value: '{}' at line: {} column: {}",id,pos.0+1,pos.1+1);
                    return Err(RuntimeError::UseOfUndeclaredVariable);
                }
            },
            _ => replaced_expr.push(tok),
        }
    }
    Ok(replaced_expr)
}

fn eval_rec<'a,'b>(input:&'a String,vars:&mut Vec<(&'b str,Token)>,entry:Vec<Token>)  -> Result<(),RuntimeError> where 'a:'b  {

    let mut tok_iter = entry.into_iter().peekable();

    while let Some(tok) = tok_iter.next() {
        match tok {
            Token::AffRal(..) => {
                print!("\n");
                tok_iter.next();// Ditch Semicolon
            },
            Token::Afficher(..) => {
                let expr_pile:Vec<Token> = tok_iter.by_ref()
                        .take_while(|&tok|!matches!(tok,Token::Semicolon(..)))
                        .collect();
                afficher(eval_expr(replace_vars(&input, vars, expr_pile)?)?)?;
            },
            Token::Identifier(s, e, _) => {
                tok_iter.next(); // Ditch =
                let op = match tok_iter.peek() {
                    Some(Token::Inv(..)) => {tok_iter.next()},
                    Some(Token::Sqrt(..)) => {tok_iter.next()},
                    _ => {None}
                };
                let expr_pile:Vec<Token> = tok_iter.by_ref()
                        .take_while(|&tok|!matches!(tok,Token::Semicolon(..)))
                        .collect();
                let first_res = eval_expr(replace_vars(&input, vars, expr_pile)?)?;
                let res = match op {
                    None => first_res,
                    Some(Token::Inv(..)) => inv(first_res)?,
                    Some(Token::Sqrt(..)) => sqrt(first_res)?,
                    Some(_) => return Err(RuntimeError::EvaluationIncoherence(line!())),
                };
                let id = input.get(s..e).unwrap_or_default();
                if let Some(pos) = vars.iter().position(|&var| var.0==id){
                    vars[pos].1 = res;
                } else {
                    vars.push((id,res));
                }
            },
            Token::Loop(..) => {
                let expr_pile:Vec<Token> = tok_iter.by_ref()
                        .take_while(|&tok|!matches!(tok,Token::OpenCurly(..)))
                        .collect();
                let iteration_nbr = match eval_expr(replace_vars(input, vars, expr_pile)?)? {
                    Token::Integer(v, _) if v >= 0 => v as u64,
                    _ => return Err(RuntimeError::IterationOverNonPositiveInteger),
                };
                
                let mut instr_pile:Vec<Token> = Vec::new();
                let mut curly:usize = 1;
                while let Some(tok) = tok_iter.next() {
                    match tok {
                        Token::CloseCurly(..) => {
                            curly-=1;
                            if curly == 0{
                                break;
                            }
                        }
                        _=> {}
                    }
                    instr_pile.push(tok);
                }
                for _ in 0..iteration_nbr {
                    eval_rec(input, vars, instr_pile.clone())?;
                }
                
            }
            _ => {println!("{} {:?}",tok,tok.pos());return Err(RuntimeError::EvaluationIncoherence(line!()));}
        }
    }


    Ok(())
}

pub fn evaluation(input:String) -> Result<(),RuntimeError> {
    
    let mut vars:Vec<(&str,Token)> = Vec::new();

    eval_rec(&input, &mut vars, Tokenizer::from(input.clone()).collect())?;

    Ok(())
}
