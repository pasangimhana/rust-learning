use std::ptr;

struct Node<T> {
    data: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            prev: ptr::null_mut(),
            next: self.head,
        }));

        if !self.head.is_null() {
            unsafe {
                (*self.head).prev = new_node;
            }
        } else {
            self.tail = new_node;
        }

        self.head = new_node;
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            prev: self.tail,
            next: ptr::null_mut(),
        }));

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = new_node;
            }
        } else {
            self.head = new_node;
        }

        self.tail = new_node;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            let old_head = Box::from_raw(self.head);
            self.head = old_head.next;

            if !self.head.is_null() {
                (*self.head).prev = ptr::null_mut();
            } else {
                self.tail = ptr::null_mut();
            }

            Some(old_head.data)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            let old_tail = Box::from_raw(self.tail);
            self.tail = old_tail.prev;

            if !self.tail.is_null() {
                (*self.tail).next = ptr::null_mut();
            } else {
                self.head = ptr::null_mut();
            }

            Some(old_tail.data)
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_front(0);

    while let Some(value) = list.pop_front() {
        println!("{}", value);
    }
}