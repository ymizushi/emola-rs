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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        use super::Tree::*;
        assert_eq!(
            Node(vec![Leaf("def"), Leaf("plus"), Node(vec![Leaf("fn"), Node(vec![Leaf("x"), Leaf("y")]), Node(vec![Leaf("+"), Leaf("x"), Leaf("y")])])])
            , parse(&mut vec!["(", "def", "plus", "(", "fn", "(", "x", "y", ")", "(", "+", "x", "y", ")", ")", ")"].iter().peekable()));
    }
}
