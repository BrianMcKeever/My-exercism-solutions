use std::iter::FromIterator;

pub struct SimpleLinkedList<T>
where
    T: std::clone::Clone,
{
    head: Option<Box<Node<T>>>,
}

struct Node<T>
where
    T: std::clone::Clone,
{
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T>
where
    T: std::clone::Clone,
{
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut current = &self.head;
        let mut count = 0;
        loop {
            match current {
                None => return count,
                Some(node) => {
                    current = &node.next;
                    count += 1;
                }
            }
        }
    }

    pub fn push(&mut self, element: T) {
        match &self.head {
            None => {
                self.head = Some(Box::new(Node {
                    data: element,
                    next: None,
                }))
            }
            Some(_) => {
                self.head = Some(Box::new(Node {
                    data: element,
                    next: self.head.take(),
                }))
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.data),
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut current = &self.head;
        loop {
            match current {
                None => return list,
                Some(node) => {
                    list.push(node.data.clone());
                    current = &node.next;
                }
            }
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T>
where
    T: std::clone::Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in iter {
            list.push(i);
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: std::clone::Clone,
{
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut reversed = self.rev();
        loop {
            match reversed.pop() {
                None => return vec,
                Some(data) => {
                    vec.push(data);
                }
            }
        }
    }
}
