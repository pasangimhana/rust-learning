use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node<T> {
    pub value: Option<T>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value: Some(value),
            prev: None,
            next: None,
        }))
    }
}

pub struct Iter<T> {
    pub current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            let next = current.borrow().next.clone();
            self.current = next;
            current
        })
    }
}

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
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.append(1);
    list.append(2);
    list.append(3);
    println!("Appended 3 elements.");

    for node in list.iter() {
        println!("{}", node.borrow().value.unwrap());
    }

    if let Some(value) = list.pop_last() {
        println!("Popped last: {}", value);
    } else {
        println!("List was empty, nothing to pop.");
    }

    if let Some(value) = list.pop_last() {
        println!("Popped last: {}", value);
    } else {
        println!("List was empty, nothing to pop.");
    }

    for node in list.iter() {
        println!("{}", node.borrow().value.unwrap());
    }

    if let Some(value) = list.pop_last() {
        println!("Popped last: {}", value);
    } else {
        println!("List was empty, nothing to pop.");
    }

    list.append(4);

    for node in list.iter() {
        println!("{}", node.borrow().value.unwrap());
    }

    if let Ok(_) = list.insert_to(0, 0) {
        println!("Value inserted successfully!");
    } else {
        println!("Invalid index!");
    }

    if let Ok(_) = list.insert_to(2, 5) {
        println!("Value inserted successfully!");
    } else {
        println!("Invalid index!");
    }

    list.append(12);
    list.append(13);

    for node in list.iter() {
        println!("{}", node.borrow().value.unwrap());
    }

    if let Ok(value) = list.pop_at(1) {
        println!("Popped at index 1: {}", value);
    } else {
        println!("Invalid index or list was empty.");
    }

    for node in list.iter() {
        println!("{}", node.borrow().value.unwrap());
    }

    // let mut list2 = DoublyLinkedList::new();

    // list2.append("hello");

    let x = Box::new("hello");

    println!("{}", x);

    let y = &x;

    println!("{}", *y);
}
