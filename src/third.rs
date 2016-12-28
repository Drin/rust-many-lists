use std::rc::Rc;

pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

struct Node<T> {
    elem: T,
    tail: Option<Rc<Node<T>>>,
}

pub struct Iter<'list_elem, T:'list_elem> {
    next: Option<&'list_elem Node<T>>,
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

    pub fn iter<'list_elem> (&'list_elem self) -> Iter<'list_elem, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'list_elem, T> Iterator for Iter<'list_elem, T> {
    type Item = &'list_elem T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.tail.as_ref().map(|node| &**node);

            &node.elem
        })
    }
}

#[cfg(test)]

mod test {
    use super::List;

    #[test]
    fn test_head() {
        let test_list = List::<i32>::new();
        assert_eq!(test_list.head(), None);

        test_list.append(5);
        assert_eq!(test_list.head(), None);

        let test_list = test_list.append(5);

        assert_eq!(test_list.head(), Some(&5));
        assert_eq!(test_list.head(), Some(&5));
    }

    #[test]
    fn test_tail_single() {
        let test_list = List::<i32>::new();

        let foo = test_list.tail();
        assert_eq!(foo.head(), None);

        let bar = test_list.append(5).tail();
        assert_eq!(bar.head(), None);
    }

    #[test]
    fn test_tail_multi() {
        let test_list = List::<i32>::new().append(5).append(12);

        let foo = test_list.tail();
        assert_eq!(foo.head(), Some(&5));

        let foo = foo.tail();
        assert_eq!(foo.head(), None);

        let bar = test_list.tail();
        assert_eq!(bar.head(), Some(&5));

        let bar = bar.tail();
        assert_eq!(bar.head(), None);

        assert_eq!(test_list.head(), Some(&12));
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
