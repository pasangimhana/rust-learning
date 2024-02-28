use std::cell::Cell;

fn main() {
    let my_number = Cell::new(42);
    let immutable_reference = &my_number;
    immutable_reference.set(immutable_reference.get() + 10);
    println!("my_number: {}", my_number.get());
}