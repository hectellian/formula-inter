use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    if !Path::new(filename).exists() {
        println!("File \"{}\" does not exist", filename);
        println!("Usage: {} \"file.fi\"", args[0]);
        return;
    }
    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                for t in Tokenizer::from(ip.clone()) {
                    match t {
                        Token::UnknownToken(s,d) => {println!("Syntaxical Error: {} is not a valid token",ip.get(s..d).unwrap()); break;},
                        Token::Identifier(s,e) => println!("Identifier({})",ip.get(s..e).unwrap()),
                        _ => println!("{}",t)
                    }
                }
                println!("");
            }
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}