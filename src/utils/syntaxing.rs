//! Semantical Analysis Module

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

#[derive(Debug)]
struct Grammar {
    s:(SToken,Vec<SToken>),
    p:Vec<(SToken,Vec<SToken>)>,
}

impl Grammar {
    pub fn our() -> Grammar {
        Grammar { 
            s: (SToken::SCRIPT,vec![SToken::LISTINSTR]), 
            p:  vec![
                (SToken::SCRIPT,vec![SToken::LISTINSTR]),
                
                (SToken::LISTINSTR,vec![SToken::INSTR,SToken::LISTINSTR]),
                (SToken::LISTINSTR,vec![SToken::EPS]),

                (SToken::INSTR,vec![SToken::TERM(Token::Identifier(0,0,0,0)),SToken::TERM(Token::Equal(0,0))]),
                
            ]
        }
    }
}

macro_rules! error_print {
    ($tok:expr,$func:expr) => {
        println!("Syntax Error: token {} found in a {} bloc",$tok,$func);   
    };
}

pub fn syntaxical_analysis(input:String) -> bool {

    let tok_stream = Tokenizer::from(input.clone());
    let mut pile:Vec<SToken> = vec![SToken::SCRIPT];

    let our_grammar: Grammar = 

    for tok in tok_stream{

    }

    true
}