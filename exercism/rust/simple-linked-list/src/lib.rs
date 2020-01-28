use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut cur_link = self.head.as_ref();
        while let Some(boxed_node) = cur_link {
            len += 1;
            cur_link = boxed_node.next.as_ref();
        }
        len
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            data: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    // pub fn rev(mut self) -> SimpleLinkedList<T> {
    //     let mut list = SimpleLinkedList::new();
    //     while let Some(elem) = self.pop() {
    //         list.push(elem);
    //     }
    //     list
    // }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut node = self.head;
        while let Some(x) = node {
            list.push(x.data);
            node = x.next;
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for elem in iter {
            list.push(elem);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    // fn into(mut self) -> Vec<T> {
    //     let mut v = vec![];
    //     while let Some(elem) = self.pop() {
    //         v.insert(0, elem);
    //     }
    //     v
    // }
    fn into(self) -> Vec<T> {
        let mut v = vec![];
        let mut node = self.head;
        while let Some(x) = node {
            v.insert(0, x.data);
            node = x.next;
        }
        v
    }
}
