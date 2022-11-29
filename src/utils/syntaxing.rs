//! Semantical Analysis Module

use super::tokens::Token;
use super::tokenizer::Tokenizer;
use std::mem::discriminant;

#[derive(Debug,Clone, Copy)]
enum SToken {
    SCRIPT,
    LISTINSTR,
    INSTR,
    PDAFF,
    E,
    D,
    T,
    G,
    F,
    TERM(Token),
    EPS
}

enum SError {
    ESCRIPT,
    ELISTINSTR,
    EINSTR,
    EPDAFF,
    EE,
    ED,
    ET,
    EG,
    EF,
    Global
}

impl std::fmt::Display for SError {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self {
            SError::ESCRIPT => write!(f, "an identifier or aff_ral, Afficher keywords"),
            SError::ELISTINSTR => write!(f, "an identifier or aff_ral, Afficher keywords"),
            SError::EINSTR => write!(f, "an identifier or aff_ral, Afficher keywords"),
            SError::EPDAFF => write!(f, "an identifier or inv keyword"),
            SError::EE => write!(f, "an identier, a number or parenthesis"),
            SError::ED => write!(f, "a plus or parenthesis"),
            SError::ET => write!(f, "an identier, a number or parenthesis"),
            SError::EG => write!(f, "a multiplication or an addition"),
            SError::EF => write!(f, "an identier, a number or parenthesis"),
            _ => write!(f,"Global error ocurred")
        }
    }
}

type GRule = (SToken,Vec<SToken>);

macro_rules! grule {
    ($stok:expr,$x:tt) => {
        ($stok,vec!$x)
    };
}

macro_rules! term {
    ($tok:ident) => {
        SToken::TERM(Token::$tok(0,0))
    };
}

fn table(sym:SToken,sym_term:Token) -> Result<GRule,SError> {
    match sym {
        SToken::SCRIPT => {
            match sym_term {
                Token::Identifier(..)|Token::AffRal(..)|Token::Afficher(..) => {
                    return Ok(grule!(SToken::SCRIPT,[ SToken::LISTINSTR ]));
                }
                _ => return Err(SError::ESCRIPT)
            }
        },
        SToken::LISTINSTR => {
            match sym_term {
                Token::Identifier(..)|Token::AffRal(..)|Token::Afficher(..) => {
                    return Ok(grule!(SToken::LISTINSTR,[SToken::INSTR,SToken::LISTINSTR]));
                }
                _ => return Err(SError::ELISTINSTR)
            }
        },
        SToken::INSTR => {
            match sym_term {
                Token::Identifier(..) => {
                    return Ok(grule!(SToken::INSTR,[SToken::TERM(Token::Identifier(0,0,0,0)),term!(Equal),SToken::PDAFF,term!(Semicolon)]));
                },
                Token::AffRal(..) => {
                    return Ok(grule!(SToken::INSTR,[term!(AffRal),term!(Semicolon)]));
                },
                Token::Afficher(..) => {
                    return Ok(grule!(SToken::INSTR,[term!(Afficher),SToken::E,term!(Semicolon)]));
                },
                _ => return Err(SError::EINSTR)
            }
        },
        SToken::PDAFF => {
            match sym_term {
                Token::Inv(..) => {
                   return Ok(grule!(SToken::PDAFF,[term!(Inv),SToken::E]));
                },
                Token::Identifier(..)|Token::Real(..)|Token::Integer(..)|Token::OpenParenthesis(..) => {
                    return Ok(grule!(SToken::PDAFF,[SToken::E]));
                },
                _ => return Err(SError::EPDAFF)
            }
        },
        SToken::E => {
            match sym_term {
                Token::Identifier(..)|Token::Real(..)|Token::Integer(..)|Token::OpenParenthesis(..) => {
                    return Ok(grule!(SToken::E,[SToken::T,SToken::D]));
                },
                _ => return Err(SError::EE)
            }
        },
        SToken::D => {
            match sym_term {
                Token::Adder(..) => {
                    return Ok(grule!(SToken::D,[term!(Adder),SToken::E]));
                },
                Token::CloseParenthesis(..)|Token::Semicolon(..) => {
                    return Ok(grule!(SToken::D,[SToken::EPS]));
                },
                _ => return Err(SError::ED)
            }
        },
        SToken::T => {
            match sym_term {
                Token::Identifier(..)|Token::Real(..)|Token::Integer(..)|Token::OpenParenthesis(..) => {
                    return Ok(grule!(SToken::T,[SToken::F,SToken::G]));
                },
                _ => return Err(SError::ET)
            }
        },
        SToken::G => {
            match sym_term {
                Token::Multiplier(..) => {
                    return Ok(grule!(SToken::G,[term!(Multiplier),SToken::T]));
                },
                Token::Adder(..)|Token::CloseParenthesis(..)|Token::Semicolon(..) => {
                    return  Ok(grule!(SToken::G,[SToken::EPS]));
                },
                _ => return Err(SError::EG)
            }
        },
        SToken::F => {
            match sym_term {
                Token::Identifier(..) => {
                    return Ok(grule!(SToken::F,[SToken::TERM(Token::Identifier(0,0,0,0))]));
                },
                Token::Real(..) => {
                    return Ok(grule!(SToken::F,[SToken::TERM(Token::Real(0.0,0,0))]));
                },
                Token::Integer(..) => {
                    return Ok(grule!(SToken::F,[SToken::TERM(Token::Integer(0,0,0))]));
                },
                Token::OpenParenthesis(..) => {
                    return Ok(grule!(SToken::F,[term!(OpenParenthesis),SToken::E,term!(CloseParenthesis)]));
                },
                _ => return Err(SError::EF)
            }
        }
        _ => return Err(SError::Global)
    }
}

macro_rules! error_print {
    ($line:expr,$col:expr,$tok:expr,$func:expr) => {
        println!("Syntax Error in line {} at col {}: found {} when expected {}",$line,$col,$tok,$func);   
    };
}

pub fn syntaxical_analysis(input:String) -> bool {

    let tok_stream = Tokenizer::from(input.clone());
    let mut pile:Vec<SToken> = vec![SToken::SCRIPT];

    for tok in tok_stream {
        loop {
            let sym = pile.pop();
            match sym {
                None => {
                    return false;
                },
                Some(sym) => {
                    match sym {
                        SToken::TERM(t) => {
                            if discriminant(&t) == discriminant(&tok)  {
                                println!("Right: {} : {}",t,tok);
                                break;
                            } else {
                                let p = tok.pos();
                                error_print!(p.0+1,p.1,tok,t);
                                return false;
                            }
                        },
                        SToken::EPS => {
                            continue;
                        },
                        _ =>{
                            let rule = table(sym, tok);
                            match rule {
                                Err(e) => {
                                    let p = tok.pos();
                                    error_print!(p.0+1,p.1,tok,e);
                                    return false
                                },
                                Ok(mut r) => {
                                    r.1.reverse();
                                    pile.append(&mut r.1);
                                }
                            }
                        }   
                    }
                }
            }
        }
    }

    true
}