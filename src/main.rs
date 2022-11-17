use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod utils;

use crate::utils::lexer::*;
use crate::utils::postfixe::*;
use crate::utils::eval::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} \"file.fi\"", args[0]);
        return;
    }
    let filename = &args[1];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                interpret(ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn interpret(algebric: String) -> EvalResult {
    let l = Lexer::from(algebric);
    let t = l.tokenize().ok().unwrap();
    println!("algebric expression: {:?}\n", t);

    let p = postfixe(t);
    println!("postfixe expression: {:?}\n", p);

    let e = eval(p);
    println!("evalued expression result: {:?}", e);
    
    e
}

#[cfg(test)]
mod test;