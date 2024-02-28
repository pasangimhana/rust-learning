use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    data: RefCell<Option<i32>>,
    next: Option<Rc<Node>>,
}

impl Node {
    fn new() -> Node {
        Node {
            data: RefCell::new(None),
            next: None,
        }
    }
}

pub fn new_with_link(next: Option<Rc<Node>>) -> Node {
    Node {
        data: RefCell::new(None),
        next,
    }
}

pub struct CircularBuffer {
    start: Option<Rc<Node>>,
    write_buffer: Option<Rc<Node>>,
    read_buffer: Option<Rc<Node>>,
    size: usize,
    capacity: usize,
}
impl CircularBuffer {
    pub fn new(capacity: usize) -> CircularBuffer {
        let start = Rc::new(Node::new());
        let mut next = Rc::new(new_with_link(Some(start.clone())));

        for _ in 1..capacity {
            let node = Rc::new(new_with_link(Some(next.clone())));
            next = node;
        }

        // connect the last node to the start node
        next = Rc::new(new_with_link(Some(start.clone())));

        let temp_start = start.clone();

        CircularBuffer {
            start: Some(start),
            write_buffer: Some(temp_start.clone()),
            read_buffer: Some(temp_start.clone()),
            size: 0,
            capacity,
        }
    }
}

fn main() { }
