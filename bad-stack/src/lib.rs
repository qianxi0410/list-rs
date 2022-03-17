use std::mem;

pub struct Stack<T: Clone + Copy> {
    head: Link<T>,
}

enum Link<T: Clone + Copy> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T: Clone + Copy> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T>
where
    T: Copy + Clone,
{
    pub fn new() -> Self {
        Stack { head: Link::Empty }
    }

    // add an element to the top of the stack
    // <- top tail ->
    pub fn push(&mut self, elem: T) {
        let new_ndoe = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_ndoe);
    }

    // remove the top element from the stack
    // <- top tail ->
    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> Drop for Stack<T>
where
    T: Copy + Clone,
{
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Stack;

    #[test]
    fn test_bad_stack() {
        let mut stack = Stack::new();
        assert!(stack.pop().is_none());

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.pop().is_none());
    }
}
