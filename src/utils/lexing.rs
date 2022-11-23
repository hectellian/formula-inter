//! Lexing Analysis module

use super::tokens::Token;
use super::tokenizer::Tokenizer;

pub fn lexical_anlysis(input: String) {
    for tok in Tokenizer::from(input.clone()) {
        match tok {
            Token::UnknownToken(s,e,l,c) => {
                println!("Syntaxe Error on line {} at column {}\n\tUnknown Tokwn: {}",l,c,input.get(s..e).unwrap())
            },
            _ => {}
        }
    }
}