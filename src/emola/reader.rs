use super::token::tokenize;
use super::parse::{parse, Tree};

use std::io;

fn read<'a>() {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();
    let tokenized: Vec<String> = tokenize(&buff);
    let mut tokens = tokenized.iter().peekable();
   // parse(&mut tokens)
}
