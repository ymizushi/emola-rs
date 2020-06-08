use std::cmp::PartialEq;
use std::cell::RefCell;
use std::collections::HashMap;

use super::parse::Tree;

#[derive(Clone, PartialEq, Debug)]
pub struct Env<'a> {
    pub map: HashMap<String, Value<'a>>,
}

impl<'a> Env<'a> {
    fn get(&self, key: String) -> Option<&Value<'a>> {
        self.map.get(&key)
    }

    fn insert(&mut self, key: String, v: Value<'a>) {
        self.map.insert(key, v);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Symbol(&'a str),
    Callable(Tree<&'a str>),
    Int(i32),
    Bool(bool),
    Nil,
}

use std::fmt;

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "Value: {}", s),
            Value::Symbol(s) => write!(f, "Value: {}", s),
            Value::Callable(_) => write!(f, "Value: {}", "Fn"),
            Value::Int(i) => write!(f, "Value: {}", i),
            Value::Bool(b) => write!(f, "Value: {}", b),
            Value::Nil => write!(f, "Value: {}", "Nill")

        }
        
    }
}

pub fn eval<'a>(t: &Tree<&'a str>, env: &'a RefCell<Env<'a>>) -> Value<'a> {
    use Tree::*;
    match t {
        Leaf(l) => {
            to_value(l, env)
        },
        Node(v) => {
            match v[0].clone() {
                Leaf("begin") => {
                    begin_exp(v[1..].to_vec(), env)
                },
                Leaf("if") => {
                    if_exp(v[1..].to_vec(), env)
                },
                Leaf("=") => {
                    equal_exp(v[1..].to_vec(), env)
                },
                Leaf("+") => {
                    adder(v[1..].to_vec(), env)
                },
                Leaf("-") => {
                    subber(v[1..].to_vec(), env)
                },
                Leaf("*") => {
                    muller(v[1..].to_vec(), env)
                },
                Leaf("/") => {
                    divider(v[1..].to_vec(), env)
                },
                Leaf("fn") => {
                    function(v[..].to_vec(), env)
                },
                Leaf("def") => {
                    define(v[1..].to_vec(), env)
                },
                Leaf(s) => {
                    match env.borrow_mut().get(String::from(s)) {
                        Some(value) => {
                            match value {
                                // Value::Callable(v) => {
                                //     exec(*v, env)
                                // }
                                _ => panic!("unknown syntax")
                            }
                        },
                        None => panic!("unknown keyword")
                    }
                }
                Node(n) => {
                    Value::Nil
                }

            }
        }
    }
}

fn to_value<'a>(l: &'a str, env: &'a RefCell<Env<'a>>) -> Value<'a> {
    if l.starts_with("\"") {
        Value::String(l)
    } else {
        match l.parse() {
            Ok(x) => Value::Int(x),
            Err(_) => {
                if l == "true" {
                    Value::Bool(true)
                } else if l == "false" {
                    Value::Bool(false)
                } else {
                    match env.borrow_mut().get(String::from(l)) {
                        Some(s) => s.clone(),
                        None => panic!("Unknown name")
                    }
                }
            }
        }
    }
}


fn adder<'a>(v: Vec<Tree<&'a str>>, ev: &'a RefCell<Env<'a>>) -> Value<'a> {
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

fn subber<'a>(v: Vec<Tree<&'a str>>, ev: &'a RefCell<Env<'a>>) -> Value<'a> {
    v.iter()
        .map(|x| eval(x, ev))
        .map(|x| {
            match x {
                Value::Int(i) => i,
                _ => panic!("")
            }
        })
        .enumerate()
        .fold(Value::Int(0), |acc, (index, x)| {
            match acc {
                Value::Int(i) =>  {
                    if index == 0 {
                        Value::Int(i+x)
                    } else {
                        Value::Int(i-x)
                    }
                }
                _ => panic!("")
            }
        })
}

fn muller<'a>(v: Vec<Tree<&'a str>>, ev: &'a RefCell<Env<'a>>) -> Value<'a> {
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
                Value::Int(i) => Value::Int(i*x),
                _ => panic!("")
            }
        })
}


fn divider<'a>(v: Vec<Tree<&'a str>>, ev: &'a RefCell<Env<'a>>) -> Value<'a> {
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
                Value::Int(i) => Value::Int(i/x),
                _ => panic!("")
            }
        })
}
fn function<'a>(v: Vec<Tree<&'a str>>, env: &'a RefCell<Env<'a>>) -> Value<'a> {
    Value::Callable(Tree::Node(v))
}

fn begin_exp<'a>(v: Vec<Tree<&'a str>>, env: &'a RefCell<Env<'a>>) -> Value<'a> {
    for (i, e) in v.iter().enumerate() {
        if i == v.len() -1 {
            return eval(&e, env)
        } else {
            eval(&e, env);
        }
    }
    Value::Nil
}

fn if_exp<'a>(v: Vec<Tree<&'a str>>, env: &'a RefCell<Env<'a>>) -> Value<'a> {
    if eval(&v[0], env) == Value::Bool(true) {
        eval(&v[1], env)
    } else {
        eval(&v[2], env)
    }
}

fn equal_exp<'a>(v: Vec<Tree<&'a str>>, env: &'a RefCell<Env<'a>>) -> Value<'a> {
    if eval(&v[0], env) == eval(&v[1], env) {
        Value::Bool(true)
    } else {
        Value::Bool(false)
    }
}


// fn exec<'a>(t: Tree<&'a str>, env: &'a RefCell<Env<'a>>) -> Value<'a> {
//     let localEnv = Env { map: HashMap::new() };
//     match t {
//         Tree::Leaf(l) => panic!("invalid syntax"),
//         Tree::Node(v) => {
//             match (v[1], v[2]) {
//                 (Tree::Node(v1), Tree::Node(v2)) =>{
//                     v1.into_iter().map(|x| {
//                         match x {
//                         }
//                     })
// 
//                 }
//             }
//         }
//     }
// }

fn define<'a>(v: Vec<Tree<&'a str>>, ev: &'a RefCell<Env<'a>>) -> Value<'a> {
    match (v[0].clone(), v[1].clone()) {
        (Tree::Leaf(bind_key), tree) =>  {
            let value = eval(&tree, ev);
            ev.borrow_mut().insert(String::from(bind_key), value);
            Value::Nil
        }
        _ => panic!("invalid syntax of define")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parse::parse;
    #[test]
    fn test_eval() {
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

    #[test]
    fn test_eval_plus() {
        let env1 = &RefCell::new(Env {
            map: HashMap::new()
        });
        let result = eval(&parse(&mut vec!["(", "def", "hoge", "5", ")"].iter().peekable()), env1);
        assert_eq!(
            Value::Nil,
            result
        );
        assert_eq!(
            &Value::Int(5),
            env1.borrow().map.get(&"hoge".to_string()).unwrap()
        );

    }

    #[test]
    fn test_eval_def_fn() {
        use super::super::parse::Tree::{Node, Leaf};
        let env = &RefCell::new(Env {
            map: HashMap::new()
        });
        let result = eval(&parse(&mut vec!["(", "def", "hoge", "(", "fn", "(", "x", "y", ")", "(", "+", "x", "y", ")", ")", ")"].iter().peekable()), env);

        assert_eq!(
            Value::Nil,
            result
        );
    }
    #[test]
    fn test_eval_if() {
        use super::super::parse::Tree::{Node, Leaf};
        let env = &RefCell::new(Env {
            map: HashMap::new()
        });
        let result = eval(&parse(&mut vec!["(", "if", "(", "=", "1", "1", ")", "(", "+", "1", "2", ")", "(", "-", "1", "2", ")", ")"].iter().peekable()), env);

        assert_eq!(
            Value::Int(3),
            result
        );

        let result2 = eval(&parse(&mut vec!["(", "if", "(", "=", "1", "2", ")", "(", "+", "1", "2", ")", "(", "-", "1", "2", ")", ")"].iter().peekable()), env);
        assert_eq!(
            Value::Int(-1),
            result2
        );
    }

    #[test]
    fn test_eval_begin() {
        use super::super::parse::Tree::{Node, Leaf};
        let env = &RefCell::new(Env {
            map: HashMap::new()
        });
        let result = eval(&parse(&mut vec!["(", "begin", "(", "def", "hoge", "1", ")", "(", "+", "hoge", "2", ")", ")"].iter().peekable()), env);

        assert_eq!(
            Value::Int(3),
            result
        );
    }
}


