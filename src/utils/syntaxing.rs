//! Semantical Analysis Module

use std::collections::btree_set::SymmetricDifference;

use super::tokens::Token;
use super::tokenizer::Tokenizer;

#[derive(Debug)]
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

#[derive(Debug)]
struct Grammar {
    s:GRule,
    p:Vec<GRule>,
}

impl Grammar {
    pub fn our() -> Grammar {
        Grammar { 
            s: grule!(SToken::SCRIPT,[ SToken::LISTINSTR ]), 
            p:  vec![
                grule!(SToken::SCRIPT,[ SToken::LISTINSTR ]),   
                grule!(SToken::LISTINSTR,[SToken::INSTR,SToken::LISTINSTR]),
                grule!(SToken::LISTINSTR,[SToken::EPS]),

                grule!(SToken::INSTR,[SToken::TERM(Token::Identifier(0,0,0,0)),term!(Equal),SToken::PDAFF,term!(Semicolon)]),
                grule!(SToken::INSTR,[term!(AffRal),term!(Semicolon)]),
                grule!(SToken::INSTR,[term!(Afficher),SToken::E,term!(Semicolon)]),

                grule!(SToken::PDAFF,[SToken::E]),
                grule!(SToken::PDAFF,[term!(Inv),SToken::E]),

                grule!(SToken::E,[SToken::T,SToken::D]),

                grule!(SToken::D,[term!(Adder),SToken::E]),
                grule!(SToken::D,[SToken::EPS]),

                grule!(SToken::T,[SToken::F,SToken::G]),

                grule!(SToken::G,[term!(Multiplier),SToken::T]),
                grule!(SToken::G,[SToken::EPS]),

                grule!(SToken::F,[term!(OpenParenthesis),SToken::E,term!(CloseParenthesis)]),
                grule!(SToken::F,[SToken::TERM(Token::Real(0.0,0,0))]),//Slight modification compared to the original grammar
                grule!(SToken::F,[SToken::TERM(Token::Integer(0,0,0))]),
                grule!(SToken::F,[SToken::TERM(Token::Identifier(0,0,0,0))])
            ]
        }
    }

    pub fn eps(self,sym:SToken) -> bool {
        return matches!(sym,SToken::G|SToken::D|SToken::LISTINSTR|SToken::SCRIPT);
    }

    pub fn table(self,sym:SToken,sym_term:Token) -> Option<GRule> {
        match sym {
            SToken::SCRIPT => {
                match sym_term {
                    Token::Identifier(..)|Token::AffRal(..)|Token::Afficher(..) => {
                        return Some(grule!(SToken::SCRIPT,[ SToken::LISTINSTR ]));
                    }
                    _ => return None
                }
            },
            SToken::LISTINSTR => {
                match sym_term {
                    Token::Identifier(..)|Token::AffRal(..)|Token::Afficher(..) => {
                        return Some(grule!(SToken::LISTINSTR,[SToken::INSTR,SToken::LISTINSTR]));
                    }
                    _ => return None
                }
            },
            SToken::INSTR => {
                match sym_term {
                    Token::Identifier(..) => {
                        return Some(grule!(SToken::INSTR,[SToken::TERM(Token::Identifier(0,0,0,0)),term!(Equal),SToken::PDAFF,term!(Semicolon)]));
                    },
                    Token::AffRal(..) => {
                        return Some(grule!(SToken::INSTR,[term!(AffRal),term!(Semicolon)]));
                    },
                    Token::Afficher(..) => {
                        return Some(grule!(SToken::INSTR,[term!(Afficher),SToken::E,term!(Semicolon)]));
                    },
                    _ => return None
                }
            },
            SToken::PDAFF => {
                match sym_term {
                    Token::Inv(..) => {
                       return Some(grule!(SToken::PDAFF,[term!(Inv),SToken::E]));
                    },
                    Token::Identifier(..)|Token::Real(..)|Token::Integer(..)|Token::OpenParenthesis(..) => {
                        return Some(grule!(SToken::PDAFF,[SToken::E]));
                    },
                    _ => return None
                }
            },
            SToken::E => {
                match sym_term {
                    Token::Identifier(..)|Token::Real(..)|Token::Integer(..)|Token::OpenParenthesis(..) => {
                        return Some(grule!(SToken::E,[SToken::T,SToken::D]));
                    },
                    _ => return None
                }
            },
            SToken::D => {
                match sym_term {
                    Token::Adder(..) => {
                        return Some(grule!(SToken::D,[term!(Adder),SToken::E]));
                    },
                    Token::CloseParenthesis(..)|Token::Semicolon(..) => {
                        return Some(grule!(SToken::D,[SToken::EPS]));
                    },
                    _ => None
                }
            },
            SToken::T => {
                match sym_term {
                    Token::Identifier(..)|Token::Real(..)|Token::Integer(..)|Token::OpenParenthesis(..) => {
                        return Some(grule!(SToken::T,[SToken::F,SToken::G]));
                    },
                    _ => return None
                }
            },
            SToken::G => {
                match sym_term {
                    Token::Multiplier(..) => {
                        return Some(grule!(SToken::G,[term!(Multiplier),SToken::T]));
                    },
                    Token::Adder(..)|Token::CloseParenthesis(..)|Token::Semicolon(..) => {
                        return  Some(grule!(SToken::G,[SToken::EPS]));
                    },
                    _ => return None
                }
            },
            SToken::F => {
                match sym_term {
                    Token::Identifier(..) => {
                        return Some(grule!(SToken::F,[SToken::TERM(Token::Identifier(0,0,0,0))]));
                    },
                    Token::Real(..) => {
                        return Some(grule!(SToken::F,[SToken::TERM(Token::Real(0.0,0,0))]));
                    },
                    Token::Integer(..) => {
                        return Some(grule!(SToken::F,[SToken::TERM(Token::Integer(0,0,0))]));
                    },
                    Token::OpenParenthesis(..) => {
                        return Some(grule!(SToken::F,[term!(OpenParenthesis),SToken::E,term!(CloseParenthesis)]));
                    },
                    _ => None
                }
            }
            _ => return None
        }
    }


}

macro_rules! error_print {
    ($tok:expr,$func:expr) => {
        println!("Syntax Error: token {} found in a {} bloc",$tok,$func);   
    };
}

pub fn syntaxical_analysis(input:String) -> bool {

    // let tok_stream = Tokenizer::from(input.clone());
    // let mut pile:Vec<SToken> = vec![SToken::SCRIPT];

    // let our_grammar: Grammar = GR;

    // for tok in tok_stream{
    //     println!("{}",tok);
    // }

    println!("{:?}",grule!(SToken::SCRIPT,[ SToken::LISTINSTR ]));

    true
}