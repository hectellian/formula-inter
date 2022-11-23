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
    }else if multi_char.eq("afficher"){
        return Some( Token::Afficher);
    } else if multi_char.eq("aff_ral"){
        return Some(Token::AffRal);
    } else if multi_char.eq("inv"){
        return  Some( Token::Inv);
    } else if multi_char.chars().next().unwrap().is_ascii_alphabetic() {
        return Some( Token::Identifier(offset-multi_char.len(),offset));
    } else {
        for c in multi_char.chars() {
            if !c.is_ascii_digit() && c != '.' && c != '-' {
                return Some( Token::UnknownToken(offset-multi_char.len(),offset,line,column-multi_char.len()));
            }
        }
        if multi_char.contains('.') {
            return Some(Token::Real(multi_char.parse::<f64>().unwrap()));
        } else if multi_char.len() == 1 && multi_char.contains('-'){
            return Some( Token::UnknownToken(offset-multi_char.len(),offset,line,column-multi_char.len()));
        } else {
            return Some(Token::Integer(multi_char.parse::<i64>().unwrap()));
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == Some( Token::EOF) || self.input.is_empty() || self.codepoint_offset >= self.input.len() {
            return None;
        }

        let char_ite = self.input.get(self.codepoint_offset..).unwrap().chars();
        let mut advance = || {self.codepoint_offset += 1; self.cur_col += 1;};

        let mut multi_char_construct = String::new();
        
        macro_rules! test_construct {
            ($false:expr) => {
                if !multi_char_construct.is_empty() {
                    break;
                } else {
                    advance();
                    self.curr = Some( $false );
                }
            };
        }

        for car in char_ite {
            match car {
                '\0' => { test_construct!(Token::EOF);self.codepoint_offset-=1;self.cur_col-=1; break;},
                '\n' => { test_construct!(Token::NewLine); self.cur_col = 0; self.cur_line+= 1; break;},
                ' ' => { test_construct!(Token::EOF);},
                '\r' => { test_construct!(Token::EOF)}, // F* u windows
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

        if !multi_char_construct.is_empty() {
            self.curr = test_multi_char_construct(multi_char_construct,self.codepoint_offset,self.cur_line,self.cur_col);
        }

        self.curr

    }
}