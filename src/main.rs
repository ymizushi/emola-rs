extern crate emola_rs;

use emola_rs::emola::{parse, eval, Env};
use std::collections::HashMap;

fn main() {
    let result = eval(
        parse(&mut vec!["(", "+", "2", "5", ")"].iter().peekable()),
        &Env {
            map: HashMap::new()
        }
    );
    println!("{:?}", result);
}
