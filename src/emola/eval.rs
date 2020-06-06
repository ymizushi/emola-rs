use std::cmp::PartialEq;
use std::collections::HashMap;

use super::parse::Tree;

#[derive(PartialEq, Debug)]
pub struct Env<'a> {
    pub map: HashMap<String, Value<'a>>,
}

impl<'a> Env<'a> {
    fn find(&self, key: String) -> Option<&Value> {
        self.map.get(&key)
    }

    fn insert(&mut self, key: String, v: Value<'a>) -> Value<'a> {
        self.map.insert(key, v).unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Symbol(&'a str),
    Callable(Tree<&'a str>, Env<'a>),
    Int(i32)
}

pub fn eval<'a>(t: &Tree<&'a str>, env: &RefCell<Env<'a>>) -> Value<'a> {
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
                Leaf("fn") => {
                    function(v[1..].to_vec(), env)
                },
                Leaf("def") => {
                    define(v[1..].to_vec(), env)
                },
                _ => {
                    Value::Int(32)
                }

            }
        }
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

use std::cell::RefCell;

fn adder<'a>(v: Vec<Tree<&'a str>>, ev: &RefCell<Env<'a>>) -> Value<'a> {
    v.iter()
        .map(|x| eval(x, ev))
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

fn function<'a>(v: Vec<Tree<&'a str>>, env: &RefCell<Env<'a>>) -> Value<'a> {
    v.iter()
        .map(|x| eval(x, env))
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

fn define<'a>(v: Vec<Tree<&'a str>>, ev: &RefCell<Env<'a>>) -> Value<'a> {
    match (v[0].clone(), v[1].clone()) {
        (Tree::Leaf(bind_key), tree) =>  {
            let value = eval(&tree, ev);
            ev.borrow_mut().insert(String::from(bind_key), value)
        }
        _ => panic!("invalid syntax of define")
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
                &parse(&mut vec!["(", "+", "2", "5", ")"].iter().peekable()),
                &RefCell::new(Env {
                    map: HashMap::new()
                })
            )
        );
    }
}


