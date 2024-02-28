use std::rc::Rc;

struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    read: usize,
    write: usize,
    capacity: usize,
    usage: usize,
}


impl<T> CircularBuffer<T> {
    fn new(capacity: usize) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: vec![None; capacity],
            read: 0,
            write: 0,
            capacity: capacity,
            usage: 0,
        }
    }

    fn write(&mut self, value: T) {
        println!("Write index: {}", self.write);
        let wrapped_val = Some(value);

        if self.is_full() {
            self.buffer[self.write] = wrapped_val;
            println!(
                "Buffer is full, overwriting the earlier element with {:?}",
                value
            );
        } else {
            self.buffer[self.write] = wrapped_val;
            self.write = (self.write + 1) % self.capacity;
            self.usage += 1;
            println!("Written value: {:?}", value);
        }
    }

    fn read(&mut self) {
        println!("Read index: {}", self.read);
        if self.is_empty() {
            println!("Buffer is empty");
        }
        let value = self.buffer[self.read].take().unwrap();
        self.read = (self.read + 1) % self.capacity;
        self.usage -= 1;
        println!("Read value: {:?}", value);
    }

    fn is_empty(&self) -> bool {
        self.usage == 0 && self.buffer[self.read].is_none()
    }

    fn is_full(&self) -> bool {
        self.usage == self.capacity
    }

    fn display(&self) {
        for i in 0..self.capacity {
            if let Some(value) = &self.buffer[i] {
                print!("{:?} ", value);
            } else {
                print!("[ ] ");
            }
        }
        println!();
    }
}


struct Superman {
    name: String,
    age: u8,
    // random rc reference
    rc: std::rc::Rc<String>,
}

fn main() {
    let mut buffer = CircularBuffer::new(5);
    buffer.write(1);
    buffer.write(2);
    buffer.write(3);
    buffer.write(4);
    buffer.write(5);

    buffer.display();

    buffer.write(6);
    buffer.display();

    buffer.read();
    buffer.read();

    buffer.display();

    buffer.write(7);
    buffer.display();

    buffer.read();
    buffer.display();
}
