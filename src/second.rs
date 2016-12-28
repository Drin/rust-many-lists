// A type alias
type Link<T> = Option<Box<Node<T>>>;

// An object-style struct
pub struct List<T> {
    head: Link<T>,
}

// A tuple-style struct
// tuple-style structs expect a ; after the parenthesis
pub struct IntoIter<T> ( List<T> );

// An object-style struct
// object-style structs expect no ; after the brace
pub struct Iter<'iter_ref, T: 'iter_ref> {
    next: Option<&'iter_ref Node<T>>,
}

pub struct IterMut<'iter_ref, T: 'iter_ref> {
    next: Option<&'iter_ref mut Node<T>>,
}

// A private (protected?) object-style struct
struct Node<T> {
    elem: T,
    next: Link<T>,
}

// implementation directly on the List<T> type
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn into_iter(self) -> IntoIter<T> { IntoIter(self) }

    pub fn iter(&self) -> Iter<T> {
        // return an instantiated Iter,
        //  where attribute "next" contains an Option<&Node<T>>.
        // Note: we contain a refernce to a node since we're reading values,
        //       but we hold a reference to a composed node for convenience
        Iter { next: self.head.as_ref().map(|node| { &**node }) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| { &mut **node }) }
    }

    #[allow(unused_variables)]
    pub fn push_vec(&mut self, new_elems: Vec<T>) {
        //TODO
    }

    pub fn push(&mut self, new_elem: T) {
        let new_node = Box::new(Node {
            elem: new_elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(| node | {
            let node = *node;

            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(| top_val | { &top_val.elem })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(| top_val | { &mut top_val.elem })
    }
}

// implementation for the Iterator trait, for the IntoIter type
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    // for a tuple-style type, self refers to the instantiated tuple.
    // tuple elements can be accessed with ".<index>" notation
    fn next(&mut self) -> Option<Self::Item> { self.0.pop() }
}

// implementation for the Iterator trait, for the Iter type
impl<'iter_ref, T> Iterator for Iter<'iter_ref, T> {
    // This implementation returns a reference to element T because, in the case of objects, we
    // need to be sure we aren't moving the data (and we won't always have a copy-able type)
    type Item = &'iter_ref T;

    // Our option is of type Item, which is a type alias to a reference, so this does not have to
    // be specified as a reference in the Option's generic type
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(| node | {
            // store the next node as a reference to a composed node.
            // I assume the resolution of &* is that & says &<ref>,
            // where ref is a composed type (*<type>)
            self.next = node.next.as_ref().map(|node| &**node);

            // make sure we return a reference to the desired element
            &node.elem
        })
    }
}

impl<'iter_ref, T> Iterator for IterMut<'iter_ref, T> {
    type Item = &'iter_ref mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(| node | {
            self.next = node.next.as_mut().map(|node| &mut **node);

            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();

        while let Some(mut boxed_node) = curr_link {
            curr_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn new_list_is_empty() {
        let mut new_list = List::<i32>::new();
        let mut list_literal = List { head: None };

        assert_eq!(new_list.pop(), list_literal.pop());
    }

    #[test]
    fn pop_returns_pushed_val() {
        let mut list = List::<i32>::new();
        let test_val = 5;

        list.push(test_val);

        assert_eq!(list.pop(), Some(test_val));
    }

    #[test]
    fn empty_list_pops_none() {
        let mut list = List::<i32>::new();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_pop_empty_test() {
        let mut list = List::<i32>::new();
        let test_val = 32;
        let test_val2 = test_val * 3;

        assert_eq!(list.pop(), None);

        assert_eq!(list.push(test_val), ());
        assert_eq!(list.pop(), Some(test_val));
        assert_eq!(list.pop(), None);

        list.push(test_val);
        list.push(test_val2);
        list.push(test_val2);
        assert_eq!(list.pop(), Some(test_val2));
        assert_eq!(list.pop(), Some(test_val2));
        assert_eq!(list.pop(), Some(test_val));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek_empty() {
        let list = List::<i32>::new();

        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = List::<i32>::new();

        list.push(5);

        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.peek(), Some(&5));

        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn test_peek_mutable() {
        let mut list = List::<i32>::new();

        list.push(23);

        // this doesn't have to be tested because it doesn't even compile
        //assert_ne!(list.peek(), Some(&mut 23));

        assert_eq!(list.peek_mut(), Some(&mut 23));
    }

    #[test]
    fn test_peek_mut() {
        let mut list = List::<i32>::new();

        list.push(5);

        let peeked_val = list.peek();

        assert_eq!(peeked_val.map(| val | { val + 5 }), Some(10));
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::<i32>::new();

        list.push(5);
        list.push(10);
        list.push(15);

        let mut list_iter = list.into_iter();

        assert_eq!(list_iter.next(), Some(15));
        assert_eq!(list_iter.next(), Some(10));
        assert_eq!(list_iter.next(), Some(5));
    }

    #[test]
    fn test_iter() {
        let mut list = List::<i32>::new();

        list.push(10);
        list.push(8);
        list.push(6);

        let mut list_iter = list.iter();

        assert_eq!(list_iter.next(), Some(&6));
        assert_eq!(list_iter.next(), Some(&8));
        assert_eq!(list_iter.next(), Some(&10));
        assert_eq!(list_iter.next(), None);

        assert_eq!(list.peek(), Some(&6));

        /*
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(8));
        assert_eq!(list.pop(), Some(10));
        */
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::<i32>::new();

        list.push(2);
        list.push(4);
        list.push(8);

        // blocks to make the iterator go out of scope
        {
            let mut list_iter = list.iter_mut();

            assert_eq!(list_iter.next(), Some(&mut 8));
            assert_eq!(list_iter.next(), Some(&mut 4));
            assert_eq!(list_iter.next(), Some(&mut 2));
        }

        {
            let mut list_iter = list.iter_mut();

            assert_eq!(list_iter.next(), Some(&mut 8));
            assert_eq!(list_iter.next(), Some(&mut 4));
            assert_eq!(list_iter.next(), Some(&mut 2));
        }
    }
}
