use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link
}

impl List {
    // constructor
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // &mut because push mutates the list
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node { 
            elem, 

            // temporarily replace self.head with Link::Empty
            next: mem::replace(&mut self.head, Link::Empty), 
        });

        // replace self.head with the new head
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // we're mutating the list, the match block requires &mut self.head
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }

        // macro that panics when the program gets here
        // unimplemented!()
    }
}

// manually implementing Drop traits
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

// mod basically creates a new file inline
// annotation says test module should only be compiled when testing
#[cfg(test)]
mod test {
    // super refers to the parent of this module
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // pop empty list
        assert_eq!(list.pop(), None);

        // populate
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}