mod iter;
mod node;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use self::iter::Iter;
use self::node::Node;

pub struct DoublyLinkedList<T> {
    count: u32,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            count: 0,
            head: None,
            tail: None,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.clone(),
        }
    }
}

impl<T: std::fmt::Display> DoublyLinkedList<T> {
    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail_weak) => {
                if let Some(old_tail) = old_tail_weak.upgrade() {
                    old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                    new_node.borrow_mut().prev = Some(old_tail_weak);
                }
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }
        self.tail = Some(Rc::downgrade(&new_node));
        self.count += 1;
    }

    pub fn pop_last(&mut self) -> Option<T> {
        self.tail.take().and_then(|old_tail_weak| {
            old_tail_weak.upgrade().map(|old_tail| {
                if let Some(prev_node_weak) = old_tail.borrow_mut().prev.take() {
                    if let Some(prev_node) = prev_node_weak.upgrade() {
                        prev_node.borrow_mut().next = None;
                        self.tail = Some(prev_node_weak);
                        self.count -= 1;
                    }
                } else {
                    self.head = None;
                    self.count = 0;
                }

                Rc::try_unwrap(old_tail)
                    .ok()
                    .expect("Something went wrong")
                    .into_inner()
                    .value
                    .unwrap()
            })
        })
    }

    pub fn insert_to(&mut self, index: u32, value: T) -> Result<(), String> {
        if index >= self.count {
            return Err("Invalid index!".to_owned());
        }
        if index == 0 {
            match self.head.take() {
                Some(old_head) => {
                    let new_node = Node::new(value);

                    old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                    new_node.borrow_mut().next = Some(old_head);

                    self.head = Some(new_node);
                }
                None => {
                    self.append(value);
                }
            }
            Ok(())
        } else {
            let mut cursor = self.head.clone();

            for _ in 0..index - 1 {
                cursor = cursor.ok_or("Index out of bound!")?.borrow().next.clone();
            }

            match cursor {
                Some(current_node) => {
                    let new_node = Node::new(value);

                    match current_node.borrow_mut().next.take() {
                        Some(new_next_node) => {
                            new_node.borrow_mut().next = Some(Rc::clone(&new_next_node));
                            new_next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                        }
                        _ => {}
                    }

                    current_node.borrow_mut().next = Some(new_node);
                    self.count += 1;
                }
                None => {
                    self.append(value);
                }
            }
            Ok(())
        }
    }

    pub fn pop_at(&mut self, index: u32) -> Result<T, String> {
        if index >= self.count {
            return Err("Invalid index!".to_owned());
        }

        if index == self.count - 1 {
            return self.pop_last().ok_or("List is empty!".to_owned());
        }

        let mut cursor = self.head.clone();

        for _ in 0..=index - 1 {
            cursor = cursor.ok_or("Index out of bound!")?.borrow().next.clone();
        }

        let current_rc = cursor.ok_or("Unexpected error: Missing current node!")?;
        let mut current_node = current_rc.borrow_mut();

        let prev = current_node
            .prev
            .take()
            .and_then(|prev_weak| prev_weak.upgrade());
        let next = current_node.next.take();

        if let Some(prev_node) = prev.clone() {
            prev_node.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next_node) = next {
            next_node.borrow_mut().prev = prev.map(|node| Rc::downgrade(&node));
        } else {
            self.tail = prev.map(|node| Rc::downgrade(&node));
        }

        self.count -= 1;

        current_node
            .value
            .take()
            .ok_or("Node without a value".to_owned())
    }

    // pub fn print_list_unsafe(&self) {
    //     let mut current_ptr = match self.head {
    //         Some(ref head) => Rc::as_ptr(head) as *const RefCell<Node<T>>,
    //         None => std::ptr::null(),
    //     };

    //     print!("[");

    //     while !current_ptr.is_null() {
    //         unsafe {
    //             let current_ref = &*current_ptr;
    //             let current_borrow = current_ref.borrow();

    //             print!("{} ", current_borrow.value);

    //             current_ptr = match current_borrow.next {
    //                 Some(ref next) => Rc::as_ptr(next) as *const RefCell<Node<T>>,
    //                 None => std::ptr::null(),
    //             };
    //         }
    //     }

    //     println!("]");
    // }

    // pub fn print_list(&self) {
    //     let mut current = self.head.as_ref();

    //     while let Some(node_rc) = current {
    //         let node_ref = node_rc.borrow();
    //         print!("{} ", node_ref.value);

    //         current = node_ref.next.as_ref();
    //     }

    //     println!();
    // }
}
