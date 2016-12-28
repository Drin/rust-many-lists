use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, new_elem: i32) {
        let new_node = Box::new(Node {
            elem: new_elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => { None }

            Link::More(boxed_node) => {
                let node = *boxed_node;

                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    /*
     * my feeble attempts
    pub fn add_iterative(&self, new_elem: i32) -> Self {
        current_node: Link = &self->head;

        while (current_node) {
        }
    }

    pub fn add_recursive(&self, new_elem: i32) -> Self {
        &self.add_helper(&self->head, new_elem);
    }

    fn add_helper(&self, node: Link, new_elem: i32) -> Self {
        if
    }
    */
}

impl Drop for List {
    fn drop(&mut self) {
        // initialize current link obj to head, and null out head
        //let mut curr_link = mem::replace(&mut self.head, Link::Empty);
        let mut curr_link = Link::Empty;

        // compose current link to a Link::More obj,
        // swap the *next* link with an empty link,
        // assign the swapped out link to current link, to process in the next iteration
        while let Link::More(mut boxed_node) = curr_link {
            curr_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    use super::Link;

    use std;

    extern crate libc;
    //use libc;

    extern {
        fn je_stats_print(
            write_cb: extern fn(*const libc::c_void, *const libc::c_char),
            cbopaque: *const libc::c_void,
            opts: *const libc::c_char
        );
    }
    extern fn write_cb (_: *const libc::c_void, message: *const libc::c_char) {
        print!(
            "{}",
            String::from_utf8_lossy(
                unsafe {
                    std::ffi::CStr::from_ptr (message as *const i8).to_bytes()
                }
            )
        );
    }

    #[test]
    fn list_memory_usage() {
        //let list = List::new();

        unsafe {
            je_stats_print(
                write_cb,
                std::ptr::null(),
                std::ptr::null()
            );
        }
    }

    #[test]
    fn new_list_is_empty() {
        let mut new_list = List::new();
        let mut list_literal = List { head: Link::Empty };

        assert_eq!(new_list.pop(), list_literal.pop());
    }

    #[test]
    fn pop_returns_pushed_val() {
        let mut list = List::new();
        let test_val = 5;

        list.push(test_val);

        assert_eq!(list.pop(), Some(test_val));
    }

    #[test]
    fn empty_list_pops_none() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn push_pop_empty_test() {
        let mut list = List::new();
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
}
