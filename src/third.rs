use std::rc::Rc;

pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

struct Node<T> {
    elem: T,
    tail: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> { List { head: None } }

    pub fn append(&self, new_elem: T) -> List<T> {
        List {
            head: Some(
                Rc::new(Node {
                    elem: new_elem,
                    tail: self.head.clone(),
                })
            )
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { &node.elem })
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(| node | {
                node.tail.clone()
            }),
        }
    }
}

#[cfg(test)]

mod test {
    use super::List;

    #[test]
    fn test_head() {
        //TODO
        let test_list = List::<i32>::new(Some(5));
    }
}
