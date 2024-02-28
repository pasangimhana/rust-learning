struct CircularBuffer {
    buffer: Vec<Option<i32>>,
    read: usize,
    write: usize,
    capacity: usize,
    usage: usize,
}

impl CircularBuffer {
    fn new(capacity: usize) -> CircularBuffer {
        CircularBuffer {
            buffer: vec![None; capacity],
            read: 0,
            write: 0,
            capacity: capacity,
            usage: 0,
        }
    }

    fn write(&mut self, value: i32) {
        println!("Write index: {}", self.write);
        let wrapped_val = Some(value);

        if self.is_full() {
            self.buffer[self.write] = wrapped_val;
            println!(
                "Buffer is full, overwriting the earlier element with {}",
                value
            );
        } else {
            self.buffer[self.write] = wrapped_val;
            self.write = (self.write + 1) % self.capacity;
            self.usage += 1;
            println!("Written value: {}", value);
        }
    }

    fn read(&mut self) {
        println!("Read index: {}", self.read);
        if self.is_empty() {
            println!("Buffer is empty");
        }
        let value = self.buffer[self.read].unwrap();
        self.buffer[self.read] = None;
        self.read = (self.read + 1) % self.capacity;
        self.usage -= 1;
        println!("Read value: {}", value);
    }

    fn is_empty(&self) -> bool {
        if (self.usage == 0 && self.buffer[self.read] == None) {
            return true;
        }
        return false;
    }

    fn is_full(&self) -> bool {
        if (self.usage == self.capacity) {
            return true;
        }
        return false;
    }

    fn display(&self) { 
        for i in 0..self.capacity {
            if let Some(value) = self.buffer[i] {
                print!("{} ", value);
            }
            else {
                print!("[ ] ");
            }
        }
        println!();
    }
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
