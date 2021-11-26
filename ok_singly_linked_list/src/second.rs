struct Node<T> {
    elem: T,
    next: Link<T>,
}

// type alias
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>
}

// tuple struct
// wrap over List<T>
// iterator which yields values
pub struct IntoIter<T>(List<T>);

// iterator which yields references
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// iterator which yields mutable references
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    // constructor
    pub fn new() -> Self {
        // return Self, which is type List<T>
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node { 
            elem, 
            next: self.head.take(), 
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // match option { None => None, Some(x) => Some(y) }
        // is a common idiom encapsulated in map
        self.head.take().map(|node| {
            self.head = node.next;

            // implicity wrapped in Some variant
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // call map on an Option over a reference to the internals (as_ref)
        // instead of consuming the Option
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

// manually implementing Drop traits
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// implement the Iterator trait for IntoIter
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // use take() here so we have exclusive access to the &mut
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        /*
        writing the argument of the closure with |&mut value| doesn't specify that value is a mutable reference
        instead, it creates a pattern that will be matched against the argument to the closure
        just use |value|
        */
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}