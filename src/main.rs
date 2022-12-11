use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
mod utils;

use crate::utils::lexing::lexical_analysis;
use crate::utils::syntaxing::syntaxical_analysis;
use crate::utils::evalution::evaluation;

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
            match lexical_analysis(content.clone()) {
                Ok(_) => {
                    match syntaxical_analysis(content.clone())  {
                        Ok(..) => {
                            match evaluation(content) {
                                Ok(..) => println!("Success!"),
                                Err(e) => println!("{:?}",e)
                            }
                        },
                        Err(e) => {println!("Syntaxical Error detected:\n");print!("{}",e)}
                    }
                },
                Err(es) => {println!("Lexical Error detected:\n");es.into_iter().for_each(|e| print!("{}",e))}
            }
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

#[cfg(test)]
mod tests;