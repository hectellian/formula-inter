//! Token constructor Module

use crate::utils::tokens::*;

/** The Tokenizer object used to tokenize a given input */
#[derive(Debug,Clone)]
pub struct Tokenizer {
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

impl Tokenizer {

    pub fn from(input: String) -> Tokenizer {
        Tokenizer {
            cur_line:0,
            cur_col:0,
            codepoint_offset:0,
            curr: None,
            input:input
        }
    }
}


fn test_multi_char_construct(multi_char:String,offset:usize,line:usize,column:usize) -> Option<Token> {
    
    if multi_char.is_empty() {
        return None;
    } else if multi_char.eq("loop") {
        return Some(Token::Loop(Some((line,column-multi_char.len()))));
    }else if multi_char.eq("afficher"){
        return Some( Token::Afficher(Some((line,column-multi_char.len()))));
    } else if multi_char.eq("aff_ral"){
        return Some(Token::AffRal(Some((line,column-multi_char.len()))));
    } else if multi_char.eq("inv"){
        return  Some( Token::Inv(Some((line,column-multi_char.len()))));
    } else if multi_char.eq("racine"){
        return Some(Token::Sqrt(Some((line,column-multi_char.len()))));
    } else if multi_char.chars().next().unwrap().is_ascii_alphabetic() {
        return Some( Token::Identifier(offset-multi_char.len(),offset,Some((line,column-multi_char.len()))));
    } else {
        for c in multi_char.chars() {
            if !c.is_ascii_digit() && c != '.' && c != '-' {
                return Some( Token::UnknownToken(offset-multi_char.len(),offset,Some((line,column-multi_char.len()))));
            }
        }
        if multi_char.contains('.') {
            return Some(Token::Real(multi_char.parse::<f64>().unwrap(),Some((line,column-multi_char.len()))));
        } else if multi_char.len() == 1 && multi_char.starts_with("-"){
            return Some( Token::UnknownToken(offset-multi_char.len(),offset,Some((line,column-multi_char.len()))));
        } else {
            return Some(Token::Integer(multi_char.parse::<i64>().unwrap_or_default(),Some((line,column-multi_char.len()))));
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if matches!(self.curr,Some(Token::EOF(..))) || self.input.is_empty() || self.codepoint_offset >= self.input.len() {
            return None;
        }

        let char_ite = self.input.get(self.codepoint_offset..).unwrap().chars();

        let mut multi_char_construct = String::new();
        
        macro_rules! test_construct {
            ($false:expr) => {
                if !multi_char_construct.is_empty() {
                    break;
                } else {
                    self.codepoint_offset += 1;
                    self.cur_col += 1;
                    self.curr = Some( $false );
                }
            };
        }

        for car in char_ite {
            match car {
                '\0' => { test_construct!(Token::EOF(Some((self.cur_line,self.cur_line))));self.codepoint_offset-=1;self.cur_col-=1; break;},
                '\n' => { test_construct!(Token::EOF(None)); self.cur_col = 0; self.cur_line+= 1;},
                ' '|'\t' => { test_construct!(Token::EOF(None));},
                '\r' => { test_construct!(Token::EOF(None));}, // F* u windows
                '=' => { test_construct!(Token::Equal(Some((self.cur_line,self.cur_line)))); break;},
                '*' => { test_construct!(Token::Multiplier(Some((self.cur_line,self.cur_line))));break;},
                '+' => { test_construct!(Token::Adder(Some((self.cur_line,self.cur_line))));break;},
                ';' => { test_construct!(Token::Semicolon(Some((self.cur_line,self.cur_line))));break;},
                '(' => { test_construct!(Token::OpenParenthesis(Some((self.cur_line,self.cur_line))));break;},
                ')' => { test_construct!(Token::CloseParenthesis(Some((self.cur_line,self.cur_line))));break;},
                '{' => { test_construct!(Token::OpenCurly(Some((self.cur_line,self.cur_line))));break;},
                '}' => { test_construct!(Token::CloseCurly(Some((self.cur_line,self.cur_line))));break;},
                _ => {
                    self.codepoint_offset += 1;
                    self.cur_col += 1;
                    multi_char_construct.push(car);
                }
            }
        }

        if !multi_char_construct.is_empty() {
            self.curr = test_multi_char_construct(multi_char_construct,self.codepoint_offset,self.cur_line,self.cur_col);
        }

        if matches!(self.curr,Some(Token::EOF(..))) { // The traiéing whitespace problem
            return None;
        }

        self.curr

    }
}