use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
mod utils;

use crate::utils::tokenizer::*;
use crate::utils::tokens::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} \"file.fi\"", args[0]);
        return;
    }

    let filename = &args[1];

    let contents = read_from(filename);
    match contents {
        Ok(content) => {
            for t in Tokenizer::from(content.clone()) {
                match t {
                    Token::UnknownToken(s,d) => {println!("Syntaxical Error: {} is not a valid token",content.get(s..d).unwrap()); break;},
                    Token::Identifier(s,e) => println!("Identifier({})",content.get(s..e).unwrap()),
                    _ => println!("{}",t)
                }
            }
            println!("");
        },
        Err(e) => println!("{}", e),
    }
}

fn read_from(filename: &str) -> Result<String, std::io::Error> {
    if !Path::new(filename).exists() {
        println!("File \"{}\" does not exist", filename);
    }

    match File::open(filename) {
        Ok(mut f) => {
            let mut content = String::new();
            let result = f.read_to_string(&mut content);
            match result {
                Ok(_) => Ok(content),
                Err(err) => return Err(err),
            }
        }
        Err(err) => return Err(err),
    }
}