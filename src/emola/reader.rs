use super::token::tokenize;
use super::parse::parse;
use super::eval::{Env, eval};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;

pub fn read<'a>() {
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).unwrap();
        let tokens = tokenize(&buff);
        let tokens = tokens.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        let mut tokens = tokens.iter().peekable();
        let treee = parse(&mut tokens);

        let env = &RefCell::new(Env {
            map: HashMap::new()
        });
        let result = eval(&treee, env);
        println!("{}",result);
    }
}
