use super::parse::Tree;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Symbol(&'a str),
    Callable(Tree<&'a str>, Env<'a>),
    Int(i32)
}


pub fn eval<'a>(t: Tree<&'a str>, env: &Env) -> Value<'a> {
    use Tree::*;
    match t {
        Leaf(l) => {
            to_value(l)
        },
        Node(v) => {
            match v[0] {
                Leaf("+") => {
                    adder(v[1..].to_vec(), env)
                },
                _ => {
                    Value::Int(32)
                }

            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_eval() {
        use super::*;
        use super::super::parse::parse;
        assert_eq!(
            Value::Int(7),
            eval(
                parse(&mut vec!["(", "+", "2", "5", ")"].iter().peekable()),
                &Env {
                    map: HashMap::new()
                }
            )
        );

    }

}


use std::collections::HashMap;

#[derive(PartialEq)]
pub struct Env<'a> {
    pub map: HashMap<String, Value<'a>>,
}

use std::fmt;
impl<'a> std::fmt::Debug for Env<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       f.debug_struct("Env")
         .field("map", &"map")
         .finish()
    }
}

impl<'a> Env<'a> {
    fn find(&self, key: String) -> Option<&Value> {
        self.map.get(&key)
    }

    fn insert(&mut self, key: String, v: Value<'a>) {
        self.map.insert(key, v);
    }
}




fn to_value(l: &str) -> Value {
    if l.starts_with("\"") {
        Value::String(l)
    } else {
        match l.parse() {
            Ok(x) => Value::Int(x),
            Err(_) => Value::Symbol(l)
        }
    }
}

fn adder<'a>(v: Vec<Tree<&'a str>>, env: &Env) -> Value<'a> {
    v.iter()
        .map(|x| eval(x.clone(), &env))
        .map(|x| {
            match x {
                Value::Int(i) => i,
                _ => panic!("")
            }
        })
        .fold(Value::Int(0), |acc, x| {
            match acc {
                Value::Int(i) => Value::Int(i+x),
                _ => panic!("")
            }
        })


}


