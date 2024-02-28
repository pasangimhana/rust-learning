### Pre-requisites

##### Option in Rust

Option is a type that represents either Some value or None. It is used to handle the absence of a value. By using Option, you can avoid null pointer exceptions. 

```rust
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(i) => println!("x: {}", i),
        None => println!("x: None"),
    }

    match y {
        Some(i) => println!("y: {}", i),
        None => println!("y: None"),
    }
}
```



#### References
References are used to refer to a value without taking ownership of it. They are used to pass a value to a function without moving it. This is a concept called borrowing.


```rust
fn main() {
    let x = 5;
    let y = &x;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("*y: {}", *y);
}
```

`&x` is used to create a reference to the value of x.

`*y` is used to dereference the reference and get the value it points to.

By default, references are immutable. To create a mutable reference, use `&mut` instead of `&`.

```rust
fn main() {
    let mut x = 5;
    let y = &mut x;

    *y += 1;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("*y: {}", *y);
}
```

But there are some limitations to references.

1. References can only have EITHER one mutable reference OR any number of immutable references
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```
This will not work and give the error at r2 creation.
```
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

2. No references can outlive the data it points to.
```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
```
This will not work and give the error at println!.
```rust
error[E0597]: `x` does not live long enough
```

3. Dangling references are not allowed

> Dangling references are pointers that reference a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory.


```rust
fn main() {
    let r = dangle();
    }

    fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```

This will not work and give the error at return statement.
```rust
error[E0106]: missing lifetime specifier
```

> Lifetimes are a complex topic and will be covered in a separate note. Simply put, lifetimes are a way to ensure that references are valid for a certain scope of time.


### Pointer types in Rust

1. References 
2. Box<T> 
3. Rc<T> 
4. Arc<T> 
5. Cell<T> 
6. RefCell<T>

#### Box<T>

Box<T> is a smart pointer that allows you to store data on the heap rather than the stack. It is used when you have a piece of data whose size can't be known at compile time and you want to use it in a context that requires an exact size.

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

1. Box<T> is a pointer to a heap-allocated value of type T.

2. Box<T> is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.

```rust
fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    if *y == *z {
        println!("y and z are equal");
    }
}
```

Here `*y` and `*z` are dereferenced and compared. This is possible because Box<T> implements Deref trait.

3. Box<T> is also a smart pointer because it implements the Drop trait, which allows you to customize the code that is run when a Box<T> goes out of scope.

```rust
struct CustomSmartPointer {
data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
 
    println!("CustomSmartPointers created.");
}
```
> Drop trait is used to implement the destructor for a type. The destructor is the code that is run when an instance of the type goes out of scope.

#### Rc<T>

Rc<T> is a reference counting smart pointer. It keeps track of the number of references to a value and when the count goes to zero, the value is dropped.

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("a: {}, b: {}, c: {}", a, b, c);
}
```
1. Rc<T> is a pointer to a heap-allocated value of type T.

2. Implements `Drop`, `Deref` and `Clone` traits.

3. Rc<T> is used when you want to allocate a value on the heap and have multiple ownership of it.

4. Rules of Rc<T> are similar to references. It can only have either one mutable reference or any number of immutable references.

5. Rc<T> is not thread safe. If you want to use it in a multi-threaded environment, you should use `Arc<T>`.

#### Cell<T> and RefCell<T>

Cell<T> and RefCell<T> are used to mutate data inside an immutable reference.

Let's try to mutate a value inside an immutable reference. Here, 

```rust
fn main() {
    let my_number = 42;
    let immutable_reference = mut& my_number;
    *immutable_reference += 10;
}
```
This will give the following error.

```
`immutable_reference` is a `&` reference, so the data it refers to cannot be written
```

But if we store the value in a Cell<T> or RefCell<T>, we can mutate the value inside an immutable reference.

```rust
use std::cell::Cell;

fn main() {
    let my_number = Cell::new(42);
    let immutable_reference = &my_number;
    immutable_reference.set(immutable_reference.get() + 10);
    println!("my_number: {}", my_number.get());
}
```








