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

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf(T),
    Node(Vec<Tree<T>>)
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

struct Env {
    map: HashMap<String, Value>,
    outer: Box<Env>
}

impl Env {
    fn find(&self, key: String) -> Option<&Value> {
        self.map.get(&key)
    }

    fn insert(&mut self, key: String, v: Value) {
        self.map.insert(key, v);
    }
}

type Callable = fn(Vec<Value>) -> Value;

pub enum Value<'a> {
    String(&'a str),
    Callable(Callable)
}

fn eval<'a, A>(t: Tree<&'a str>, env: Env) -> Value {
    match t {
        Tree::Leaf(l) => Value::String(l),
        Tree::Node(v) => {
            Value::Int(32)
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

}
