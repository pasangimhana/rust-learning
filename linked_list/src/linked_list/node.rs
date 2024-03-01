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
