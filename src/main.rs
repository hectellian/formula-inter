use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod utils;

use crate::utils::lexer::*;

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
                for t in Lexer::from(ip) {
                    println!("{:?}",t)
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