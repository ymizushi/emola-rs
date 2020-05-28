use std::cell::RefCell;

pub fn tokenize(s: &str) -> Vec<String> {
    let input = String::from(s);
    let buffer = RefCell::new(String::new());
    let output: RefCell<Vec<String>> = RefCell::new(Vec::new());
    for c in input.chars() {
        match c {
            ' ' => {
                if buffer.borrow().len() != 0 {
                    if buffer.borrow().starts_with("\"") && !buffer.borrow().ends_with("\"") {
                        buffer.borrow_mut().push(c);
                    } else {
                        output.borrow_mut().push(buffer.borrow().to_string());
                        buffer.borrow_mut().clear()
                    }
                } 
            }
            '('  => {
                if buffer.borrow().starts_with("\"") {
                    buffer.borrow_mut().push(c);
                } else if buffer.borrow().len() != 0 {
                    // tokenize error
                } else { //normal case
                    buffer.borrow_mut().push(c);
                    output.borrow_mut().push(buffer.borrow().to_string());
                    buffer.borrow_mut().clear();
                }
            }
            ')' => {
                if buffer.borrow().starts_with("\"") {
                    buffer.borrow_mut().push(c);
                    if buffer.borrow().ends_with("\"") {
                        output.borrow_mut().push(buffer.borrow().to_string());
                        buffer.borrow_mut().clear();
                    }
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
            _ => {
                buffer.borrow_mut().push(c);
            }
        }
    }
    output.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec!["(", "def", "plus", "(", "fn", "(", "x", "y", ")", "(", "+", "x", "y", ")", ")", ")"]
            , tokenize("(def plus (fn (x y) (+ x y)))"));
        assert_eq!(
            vec!["(", "def", "plus", "(", "fn", "(", "x", "y", ")", "(", "+", "x", "y", ")", ")", ")"]
            , tokenize("(def plus (fn (x y) (+ \"piyo\" \"fuga\")))"));
    }
}
