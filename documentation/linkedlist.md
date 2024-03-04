src/linked_list/node.rs

```rust
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
```

src/linked_list/iter.rs

```rust
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
```

src/lib.rs

```rust
pub mod linked_list;
```

src/linked_list.rs
    
```rust
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

}
```

# Doubly Linked List with Rust

In this article, we will implement a doubly linked list in Rust. We will use the `Rc` and `RefCell` types to create a doubly linked list that can be iterated over and modified.

Common concepts that reoccur in this implementation are:

- `Rc` - A reference-counted pointer for shared ownership.
- `RefCell` - A mutable memory location with dynamically checked borrow rules.
- `Weak` - A weak version of `Rc`.
- `Option` - A type that represents an optional value.

This implementation is structured as follows:

- `Node` - A struct that represents a node in the doubly linked list.
- `DoublyLinkedList` - A struct that represents the doubly linked list.

- At every point in time, the `DoublyLinkedList` struct has a reference to the head and tail of the list. The head is a strong reference to the first node, and the tail is a weak reference to the last node. The `Node` struct has a value, a weak reference to the previous node, and a strong reference to the next node.

[[ Diagram ]]

## Node

The `Node` struct represents a node in the doubly linked list. It contains the following fields:

- `value` - An `Option` that holds the value of the node.
- `prev` - An `Option` that holds a weak reference to the previous node.
- `next` - An `Option` that holds a strong reference to the next node.

```rust
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
```

Let's go through the fields of the `Node` struct:

- `value` - An `Option` that holds the value of the node. It is an `Option` because the value can be `None` if the node is empty.

- `prev` - An `Option` that holds a weak reference to the previous node. It is an `Option` because the previous node can be `None` if the current node is the head of the list. It is a weak reference to avoid reference cycles.

- `next` - An `Option` that holds a strong reference to the next node. It is an `Option` because the next node can be `None` if the current node is the tail of the list.

The `Node` struct also has an associated function `new` that creates a new node with the given value and returns a reference-counted pointer to it.

## DoublyLinkedList

The `DoublyLinkedList` struct represents the doubly linked list. It contains the following fields:

- `count` - A `u32` that holds the count of nodes in the list.
- `head` - An `Option` that holds a strong reference to the head node.
- `tail` - An `Option` that holds a weak reference to the tail node.

```rust
struct DoublyLinkedList<T> {
    count: u32,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}
```

The `DoublyLinkedList` struct has the following methods implemented.

## Constructor / New Method - `new`

The `new` method creates a new empty doubly linked list.

```rust
impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            count: 0,
            head: None,
            tail: None,
        }
    }
}
```

Here, we create a new `DoublyLinkedList` with the count set to `0` and the head and tail set to `None`.

## Iterator - `iter`

The `iter` method returns an iterator over the nodes of the list.

```rust
impl<T> DoublyLinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.clone(),
        }
    }
}
```

The `Iter` struct is defined in a separate module and is used to iterate over the nodes of the list.

## Append Method - `append`

The `append` method appends a new node with the given value to the end of the list.

```rust
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
}
```

Here, we create a new node with the given value and append it to the end of the list. If the list is empty, the new node becomes the head and the tail of the list. If the list is not empty, we update the next and prev references of the old tail and the new node to link them together.

Let's see what happens behind the scenes and how ownership changes or borrows occure with smart pointer operations by going through line by line.

- `let new_node = Node::new(value);` - We create a new node with the given value using the `new` method of the `Node` struct. This returns a reference-counted pointer to the new node.

- `match self.tail.take() {` - We take the tail of the list using the `take` method of the `Option` type. This replaces the tail with `None` and returns the previous tail if it exists.

- `Some(old_tail_weak) => {` - If the previous tail exists, we enter this block. `Some()` function is used to match the `Some` variant of the `Option` type.

- `if let Some(old_tail) = old_tail_weak.upgrade() {` - We try to upgrade the weak reference to the previous tail to a strong reference using the `upgrade` method of the `Weak` type. If the upgrade is successful, we enter this block. Upgrading a weak reference to a strong reference returns an `Option` that contains the strong reference if it exists. We can then use the `if let` pattern to match the `Some` variant and extract the strong reference.

- `old_tail.borrow_mut().next = Some(Rc::clone(&new_node));` - We borrow the mutable reference of the previous tail using the `borrow_mut` method of the `RefCell` type and update the `next` field to point to the new node. We use the `Rc::clone` function to create a new reference-counted pointer to the new node.

- `new_node.borrow_mut().prev = Some(old_tail_weak);` - We borrow the mutable reference of the new node using the `borrow_mut` method of the `RefCell` type and update the `prev` field to point to the previous tail using the weak reference.

- `None => {` - If the previous tail does not exist, we enter this block.

- `self.head = Some(Rc::clone(&new_node));` - We set the head of the list to the new node by creating a new reference-counted pointer to it. 

- `self.tail = Some(Rc::downgrade(&new_node));` - We set the tail of the list to the new node by creating a weak reference to it using the `downgrade` method of the `Rc` type.

- `self.count += 1;` - We increment the count of the list by `1`.

To summarize the process happening in the `append` method, we create a new node with the given value. Then we remove the ta

## Pop Last Method - `pop_last`

The `pop_last` method removes the last node from the list and returns its value.

```rust
impl<T: std::fmt::Display> DoublyLinkedList<T> {
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
}
```

Here, we take the tail of the list using the `take` method of the `Option` type. This replaces the tail with `None` and returns the previous tail if it exists. We then upgrade the weak reference to the previous tail to a strong reference using the `upgrade` method of the `Weak` type. If the upgrade is successful, we remove the last node from the list by updating the `next` and `prev` references of the previous tail and the last node. We then decrement the count of the list by `1` and return the value of the last node.

Let's go through the process happening in the `pop_last` method by going through line by line.

- `self.tail.take().and_then(|old_tail_weak| {` - We take the tail of the list using the `take` method of the `Option` type. This replaces the tail with `None` and returns the previous tail if it exists. We then use the `and_then` method of the `Option` type to apply a closure to the previous tail if it exists.

- `old_tail_weak.upgrade().map(|old_tail| {` - We try to upgrade the weak reference to the previous tail to a strong reference using the `upgrade` method of the `Weak` type. If the upgrade is successful, we enter this block. Upgrading a weak reference to a strong reference returns an `Option` that contains the strong reference if it exists. We can then use the `map` method of the `Option` type to apply a closure to the strong reference.

- `if let Some(prev_node_weak) = old_tail.borrow_mut().prev.take() {` - We borrow the mutable reference of the previous tail using the `borrow_mut` method of the `RefCell` type and take the `prev` field. This replaces the `prev` field with `None` and returns the previous node if it exists. We use the `if let` pattern to match the `Some` variant and extract the weak reference.

- `if let Some(prev_node) = prev_node_weak.upgrade() {` - We try to upgrade the weak reference to the previous node to a strong reference using the `upgrade` method of the `Weak` type. If the upgrade is successful, we enter this block. Upgrading a weak reference to a strong reference returns an `Option` that contains the strong reference if it exists. We can then use the `if let` pattern to match the `Some` variant and extract the strong reference.

- `prev_node.borrow_mut().next = None;` - We borrow the mutable reference of the previous node using the `borrow_mut` method of the `RefCell` type and update the `next` field to point to `None`.

- `self.tail = Some(prev_node_weak);` - We set the tail of the list to the previous node by creating a weak reference to it using the `downgrade` method of the `Rc` type.

- `self.count -= 1;` - We decrement the count of the list by `1`.

- `Rc::try_unwrap(old_tail)` - We try to unwrap the reference-counted pointer to the previous tail using the `try_unwrap` method of the `Rc` type. This returns the inner value of the reference-counted pointer if the reference count is `1`.

- `.ok().expect("Something went wrong")` - We use the `ok` method of the `Result` type to convert the `Result` to an `Option` and the `expect` method of the `Option` type to panic with the given message if the `Option` is `None`.

- `.into_inner().value.unwrap()` - We use the `into_inner` method of the `RefCell` type to extract the inner value of the `RefCell` and the `unwrap` method of the `Option` type to extract the value if it exists.

To summarize the process happening in the `pop_last` method, we take the tail of the list and upgrade the weak reference to the previous tail to a strong reference. We then remove the last node from the list by updating the `next` and `prev` references of the previous tail and the last node. We then decrement the count of the list by `1` and return the value of the last node.