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

pub fn parse<'a>(v: &'a [&str]) -> (usize, Tree<&'a str>) {
    use Tree::*;
    let mut result = Node(vec![]);
    let mut index = 0;
    while index < v.len() {
        println!("{:?}", result);
        match v[index] {
            "(" => {
                match result {
                    Node(ref mut n) => {
                        let (end_index, t) = parse(&v[index+1..v.len()]);
                        (*n).push(t);
                        index = index + end_index;
                    },
                    _ => {}
                }
            },
            ")" => return (index, result),
            s => {
                match result {
                    Node(ref mut n) => {
                       (*n).push(Leaf(s));
                    },
                    _ => {}
                }
            }
        }
        index += 1
    }
    (v.len()-1, result)
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
        assert_eq!(
            (8, Tree::Leaf("10"))
            , parse(&["(", "def", "plus", "(", "fn", "(", "x", "y", ")", "(", "+", "x", "y", ")", ")", ")"]));
    }

}
