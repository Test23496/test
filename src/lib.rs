// Rust implementation of a doubly linked list

pub struct Node<T> {
    pub value: T,
    pub prev: Option<Box<Node<T>>>,
    pub next: Option<Box<Node<T>>>,
}

pub struct DoublyLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub tail: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            prev: self.tail.take(),
            next: None,
        });
        let new_node_ptr = Some(new_node);

        if let Some(ref mut tail) = self.tail {
            tail.next = new_node_ptr;
        } else {
            self.head = new_node_ptr;
        }
        self.tail = new_node_ptr;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            if let Some(ref mut prev) = tail.prev {
                self.tail = Some(prev);
                self.tail.as_mut().unwrap().next = None;
            } else {
                self.head = None;
            }
            self.length -= 1;
            tail.value
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }
}