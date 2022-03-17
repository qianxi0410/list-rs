type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Stack<T> {
    head: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<'a, T> Stack<T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
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

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
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

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));

        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));

        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));

        assert!(stack.peek().is_none());
        assert!(stack.pop().is_none());

        stack.push(3);
        assert_eq!(stack.peek_mut(), Some(&mut 3));

        let num = stack.peek_mut().unwrap();
        *num = 4;
        assert_eq!(stack.peek(), Some(&4));

        assert!(stack.pop().is_some());

        assert!(stack.pop().is_none());

        stack.push(5);
        stack.push(4);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);

        stack.push(6);
        let mut iter = stack.iter_mut();
        assert_eq!(iter.next(), Some(&mut 6));
    }
}
