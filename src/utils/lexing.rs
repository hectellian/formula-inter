//! Lexing Analysis module

use super::tokens::Token;
use super::tokenizer::Tokenizer;

pub struct LexicalError {
    error_token: String,
    error_position: (usize,usize),
    line_of_error: String
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
        let s = format!("{:3}| {}\n{:>space$}^ Unknown Token \x1b[93;31m{}\x1b[0m\n\n",self.error_position.0+1,self.line_of_error," ",self.error_token,space=(self.error_position.1+5));
        write!(f,"{}",s)
    }
}

impl LexicalError {
    fn from(error_token:String,error_position:(usize,usize),line_of_error:String) -> LexicalError {
        LexicalError { error_token, error_position, line_of_error}
    }
}


pub fn lexical_analysis(input: String) -> Result<(),Vec<LexicalError>> {
    let mut err_pile:Vec<LexicalError> = Vec::new();
    Tokenizer::from(input.clone()).for_each(|t| 
        if let Token::UnknownToken(s, e, p) = t {
            if let Some(p) = p {
                err_pile.push(LexicalError::from(input.get(s..e).unwrap_or_default().to_string(),p, input.lines().nth(p.0).unwrap_or_default().to_string()))
            } else {
                err_pile.push(LexicalError::from(input.get(s..e).unwrap_or_default().to_string(), (0,0),String::new()))
            }
        }
    );

    if err_pile.is_empty() {
        return Ok(());
    }

    return Err(err_pile);
}