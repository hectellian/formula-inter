//! Lexical analyzer Module

#![allow(dead_code,unused_variables)]

use crate::utils::tokens::*;

#[derive(Debug)]
pub enum LexerError {
    /** Unexpected end of file */
    UnexpectedEndOfFile,

    /** Unexpected number format */
    UnexpectedNumberFormat,

    /** Unknown Token read */
    UnknownToken,
}

/** The lexer object used to tokenize a given input */
#[derive(Debug,Clone)]
pub struct Lexer {
    /** The entry text */
    pub input: String,

    /** The Human readable line */
    pub cur_line: usize,

    /** The Human readable column */
    pub cur_col: usize,

    /** The cursor position from the start of input */
    pub codepoint_offset: usize,

    /** The current token */
    pub curr: Option<Token>,

}

impl Lexer {

    pub fn from(input: String) -> Lexer {
        Lexer {
            cur_line:0,
            cur_col:0,
            codepoint_offset:0,
            curr: None,
            input:input
        }
    }

}

fn test_multi_char_construct(multi_char:String) -> Option<Token> {
    
    if multi_char.is_empty() {
        return None;
    }else if multi_char.eq("afficher"){
        return Some( Token::Afficher);
    } else if multi_char.eq("inv"){
        return  Some( Token::Inv);
    } else if multi_char.chars().next().unwrap().is_ascii_alphabetic() {
        return Some( Token::Identifier );
    } else {
        for c in multi_char.chars() {
            if !c.is_ascii_digit() || c != '.' {
                return Some( Token::UnknownToken);
            }
        }
        if multi_char.contains('.') {
            return Some(Token::Real(multi_char.parse().unwrap()));
        } else {
            return Some(Token::Integer(multi_char.parse().unwrap()));
        }
    }
}


impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == Some( Token::EOF) || self.input.is_empty() || self.codepoint_offset >= self.input.len() {
            return None;
        }

        let char_ite = self.input.get(self.codepoint_offset..).unwrap().chars();

        let mut advance = || {self.codepoint_offset += 1; self.cur_col += 1;};

        let mut multi_char_construct = String::new();
        let mut construct = None;
        
        macro_rules! test_construct {
            ($false:expr) => {
                construct = test_multi_char_construct(multi_char_construct);
                if construct.is_some(){
                    self.curr = construct;
                    break;
                } else {
                    advance();
                    self.curr = Some( $false )
                }
            };
        }

        for car in char_ite {
            match car {
                '\0' => { test_construct!(Token::EOF);self.codepoint_offset-=1;self.cur_col-=1; break;},
                '\n' => { test_construct!(Token::NewLine); self.cur_col = 0; self.cur_line+= 1; break;},
                ' ' => { test_construct!(Token::EOF);},
                '=' => { test_construct!(Token::Equal); break;},
                '*' => { test_construct!(Token::Operator { raw: '*', kind:OperatorKind::Multiplier});break;},
                '+' => { test_construct!(Token::Operator { raw: '+', kind: OperatorKind::Adder });break;},
                ';' => { test_construct!(Token::Semicolon);break;},
                '(' => { test_construct!(Token::OpenParenthesis);break;},
                ')' => { test_construct!(Token::CloseParenthesis);break;},
                _ => {
                    advance();
                    multi_char_construct.push(car);
                }
            }
        }

        self.curr

    }
}