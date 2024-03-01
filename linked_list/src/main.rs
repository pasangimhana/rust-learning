use std::borrow::BorrowMut;

use linked_list::linked_list::DoublyLinkedList;

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
