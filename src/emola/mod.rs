use std::cell::RefCell;

pub fn tokenize(s: &str) -> Vec<String> {
    let input = String::from(s);
    let buffer = RefCell::new(String::new());
    let output: RefCell<Vec<String>> = RefCell::new(Vec::new());
    for c in input.chars() {
        match c {
            '"' => {
                if buffer.borrow().starts_with("\"") {
                    buffer.borrow_mut().push(c);
                    output.borrow_mut().push(buffer.borrow().to_string());
                    buffer.borrow_mut().clear()
                } else {
                    buffer.borrow_mut().push(c);
                }
            }
            '('  => {
                if buffer.borrow().starts_with("\"") {
                    buffer.borrow_mut().push(c);
                } else { //normal case
                    buffer.borrow_mut().push(c);
                    output.borrow_mut().push(buffer.borrow().to_string());
                    buffer.borrow_mut().clear();
                }
            }
            ')' => {
                if buffer.borrow().starts_with("\"") {
                    buffer.borrow_mut().push(c);
                } else { //normal case
                    if buffer.borrow().len() != 0 {
                        output.borrow_mut().push(buffer.borrow().to_string());
                        buffer.borrow_mut().clear();
                    } 
                    buffer.borrow_mut().push(c);
                    output.borrow_mut().push(buffer.borrow().to_string());
                    buffer.borrow_mut().clear();
                }
            }
            ' ' => {
                if buffer.borrow().starts_with("\"") {
                    buffer.borrow_mut().push(c);
                } else { //normal case
                    if buffer.borrow().len() != 0 {
                        output.borrow_mut().push(buffer.borrow().to_string());
                    }
                    buffer.borrow_mut().clear();
                }
            }
            _ => {
                buffer.borrow_mut().push(c);
            }
        }
    }
    output.into_inner()
}


pub fn parse<'a>(iterator: &mut std::iter::Peekable<std::slice::Iter<'_, &'a str>>) -> Tree<&'a str> {
    use Tree::*;
    let first = iterator.next().unwrap();
    match first {
        &"(" => {
            let mut node = Node(vec![]);
            match node {
                Node(ref mut iv) => {
                    loop {
                        if iterator.peek().unwrap() == &&")" {
                            iterator.next();
                            return node
                        } else {
                            iv.push(parse(iterator));
                        }
                    }
                },
                _ => {
                    panic!("")
                }
            }
        }
        &")" => {
            panic!("unknowo error");
        }
        _ => {
            return Leaf(first)
        }
    }
}

use std::collections::HashMap;

struct Env<'a> {
    map: HashMap<String, Value<'a>>,
}

impl<'a> Env<'a> {
    fn find(&self, key: String) -> Option<&Value> {
        self.map.get(&key)
    }

    fn insert(&mut self, key: String, v: Value<'a>) {
        self.map.insert(key, v);
    }
}

type Callable = fn(Vec<Value>) -> Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>)
}

use std::cmp::PartialEq;
#[derive(Clone)]
pub enum Value<'a> {
    String(&'a str),
    Symbol(&'a str),
    Callable(Callable),
    Int(i32)
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::String(_)| Self::Int(_) => {
                self == other
            }
            _ => false
        }
    }
}

use std::fmt;
impl<'a> std::fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Self::String(s) => String::from(""),
            Self::Callable(s) => String::from("callable"),
            Self::Int(i) => String::from(""),
            Self::Symbol(s) => String::from("")
        };
       f.debug_struct("Value")
         .field("content", &result)
         .finish()
    }
}

fn eval<'a>(t: Tree<&'a str>, env: &Env) -> Value<'a> {
    use Tree::*;
    match t {
        Leaf(l) => {
            if l.starts_with("\"") {
                Value::String(l)
            } else {
                match l.parse() {
                    Ok(x) => Value::Int(x),
                    Err(_) => Value::Symbol(l)
                }
            }
        },
        Node(v) => {
            match v[0] {
                Leaf("+") => {
                    v[1..].iter()
                        .map(|x| eval(x.clone(), env))
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
    use super::*;
    #[test]
    fn test_tokenize() {
        assert_eq!(
            vec!["(", "def", "plus", "(", "fn", "(", "x", "y", ")", "(", "+", "x", "y", ")", ")", ")"]
            , tokenize("(def plus (fn (x y) (+ x y)))"));
        assert_eq!(
            vec!["(", "def", "plus", "(", "fn", "(", "x", "y", ")", "(", "+", "\"piyo\"", "\"fuga\"", ")", ")", ")"]
            , tokenize("(def plus (fn (x y) (+ \"piyo\" \"fuga\")))"));
    }

    #[test]
    fn test_parse() {
        use super::Tree::*;
        assert_eq!(
            Node(vec![Leaf("def"), Leaf("plus"), Node(vec![Leaf("fn"), Node(vec![Leaf("x"), Leaf("y")]), Node(vec![Leaf("+"), Leaf("x"), Leaf("y")])])])
            , parse(&mut vec!["(", "def", "plus", "(", "fn", "(", "x", "y", ")", "(", "+", "x", "y", ")", ")", ")"].iter().peekable()));
    }
    #[test]
    fn test_eval() {
        use super::Value::*;
        assert_eq!(
            Value::Int(32),
            eval(
                parse(&mut vec!["(", "+", "1", "2", ")"].iter().peekable()),
                &Env {
                    map: HashMap::new()
                }
            )
        );

    }

}
