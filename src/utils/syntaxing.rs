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

#[derive(Debug)]
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
    ETERM(Token),
    Global
}

impl std::fmt::Display for SError {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self {
            SError::ESCRIPT => write!(f, "an identifier or aff_ral, Afficher, loop keywords (ESCRIPT)"),
            SError::ELISTINSTR => write!(f, "an identifier or aff_ral, Afficher, loop keywords (ELISTINSTR)"),
            SError::EINSTR => write!(f, "an identifier or aff_ral, Afficher, loop keywords (EINSTR)"),
            SError::EPDAFF => write!(f, "an identifier or inv, sqrt keyword (EPADFF)"),
            SError::EE => write!(f, "an identier, a number or parenthesis (EE)"),
            SError::ED => write!(f, "a plus or parenthesis (ED)"),
            SError::ET => write!(f, "an identier, a number or parenthesis (ET)"),
            SError::EG => write!(f, "a multiplication or an addition (EG)"),
            SError::EF => write!(f, "an identier, a number or parenthesis (EF)"),
            SError::ETERM(t) => write!(f,"{} (ETERM)",t),
            _ => write!(f,"Global error ocurred")
        }
    }
}

#[derive(Debug)]
pub struct SyntaxicalError {
    error_type: SError,
    token_found: Option<String>,
    error_position:(usize,usize),
    line_of_error: String
}

impl SyntaxicalError {
    fn from(error_type:SError,token_found:Option<String>,error_position:(usize,usize),line_of_error:String) -> SyntaxicalError {
        SyntaxicalError { error_type, token_found, error_position, line_of_error}
    }
}

impl std::fmt::Display for SyntaxicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stok = match &self.token_found {
            Some(s) => s.to_string(),
            None => String::from("Nothing"),
        };
        let s = format!("{:3}| {}\n{:>space$}^ Found \x1b[93;31m{}\x1b[0m, {} were expected\n\n",self.error_position.0+1,self.line_of_error," ",stok,self.error_type,space=(self.error_position.1+5));
        write!(f,"{}",s)
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
                Token::Identifier(..)|Token::AffRal(..)|Token::Afficher(..)|Token::Loop(..) => {
                    return Ok(grule!(SToken::SCRIPT,[ SToken::LISTINSTR ]));
                }
                _ => return Err(SError::ESCRIPT)
            }
        },
        SToken::LISTINSTR => {
            match sym_term {
                Token::Identifier(..)|Token::AffRal(..)|Token::Afficher(..)|Token::Loop(..)=> {
                    return Ok(grule!(SToken::LISTINSTR,[SToken::INSTR,SToken::LISTINSTR]));
                },
                Token::CloseCurly(..) => {
                    return Ok(grule!(SToken::LISTINSTR,[SToken::EPS]));
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
                Token::Loop(..) => {
                    return Ok(grule!(SToken::INSTR,[term!(Loop),SToken::E,term!(OpenCurly),SToken::LISTINSTR,term!(CloseCurly)]));
                }
                _ => return Err(SError::EINSTR)
            }
        },
        SToken::PDAFF => {
            match sym_term {
                Token::Inv(..) => {
                   return Ok(grule!(SToken::PDAFF,[term!(Inv),SToken::E]));
                },
                Token::Sqrt(..) => {
                    return Ok(grule!(SToken::PDAFF,[term!(Sqrt),SToken::E]));
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
                Token::CloseParenthesis(..)|Token::Semicolon(..)|Token::OpenCurly(..) => {
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
                Token::Adder(..)|Token::CloseParenthesis(..)|Token::Semicolon(..)|Token::OpenCurly(..) => {
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

pub fn syntaxical_analysis(input:String) -> Result<(),SyntaxicalError>{

    let mut pile:Vec<SToken> = vec![SToken::SCRIPT];

    for tok in Tokenizer::from(input.clone()) {
        while let Some(sym) = pile.pop() {
            match sym {
                SToken::TERM(t) => {
                    if discriminant(&t) == discriminant(&tok) {
                        break;
                    }

                    let p = tok.pos().unwrap_or((0,0));
                    let stok = match tok {
                        Token::Identifier(s,e,..) => input.get(s..e).unwrap_or_default().to_string(),
                        _ => format!("{}",tok)
                    };
                    return Err(SyntaxicalError::from(SError::ETERM(t), Some(stok), p, input.lines().nth(p.0).unwrap_or_default().to_string()));
                },
                SToken::EPS => {
                    continue;
                },
                _ => {
                    match table(sym, tok) {
                        Err(e) => {
                            let p = tok.pos().unwrap_or((0,0));
                            let stok = match tok {
                                Token::Identifier(s,e,..) => input.get(s..e).unwrap_or_default().to_string(),
                                _ => format!("{}",tok)
                            };
                            return Err(SyntaxicalError::from(e, Some(stok), p, input.lines().nth(p.0).unwrap_or_default().to_string()));
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
    
    let l = input.lines().count();
    if let Some(err_line) = input.lines().last() {

        macro_rules! err {
            ($tok:ident) => {
                Err(SyntaxicalError::from(SError::$tok, None, (l,err_line.len()), err_line.to_string()))
            };
        }

        while let Some(st) = pile.pop() {
            match st {
                SToken::F => return err!(EF),
                SToken::E => return err!(EE),
                SToken::INSTR => return err!(EINSTR),
                SToken::PDAFF => return err!(EPDAFF),
                SToken::T => return err!(ET),
                _ => {}
            }
        }
    }
    
    Ok(())
}