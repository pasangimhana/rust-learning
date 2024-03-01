use std::{cell::RefCell, rc::Rc};

use super::node::Node;

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
