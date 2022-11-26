use std::collections::btree_map::Entry;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
mod utils;

use utils::tokens::Token;

use crate::utils::lexing::lexical_anlysis;
use crate::utils::syntaxing::f;
use crate::utils::tokenizer::Tokenizer;

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
            lexical_anlysis(content.clone());
            let entry:Vec<Token> = Tokenizer::from(content.clone()).collect();
            let entry_len = entry.len();
            f(entry,0,entry_len);
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