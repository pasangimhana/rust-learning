## Option in Rust
Option is a type that represents either Some value or None. It is used to handle the absence of a value. By using Option, you can avoid null pointer exceptions. Because Option is an enum, it can be used with match to handle the Some and None cases.

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

### Methods of Option

- `Option::is_some()` - Returns true if the Option is Some.

- `Option::is_none()` - Returns true if the Option is None.

- `Option::unwrap()` - Returns the value inside the Option if it is Some. If it is None, it panics.

- `Option::expect()` - Returns the value inside the Option if it is Some. If it is None, it panics with the message passed to it.

- `Option::unwrap_or()` - Returns the value inside the Option if it is Some. If it is None, it returns the value passed to it.

## References
References are used to refer to a value without taking ownership of it. They are used to pass a value to a function without moving it. This is a concept called *borrowing*.

> Note : Reference & in Rust is different from C/C++ reference &. In Rust, & is used to create a reference to a value and * is used to dereference the reference. In C/C++, & is used to get the address of a variable and * is used to get the value at the address. In Rust, the address of a variable is not exposed to the programmer.

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

#### Mutable and Immutable References

- By default, references are immutable. This means that you can't change the value that a reference points to.

- To create a mutable reference, use `&mut` instead of `&`.

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

Here `&mut x` creates a mutable reference to the value of x. `*y` is used to dereference the reference and get the value it points to. `*y += 1` increments the value that `y` points to.

But there are some limitations to references.

1. References can only have EITHER one mutable reference OR any number of immutable references.
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

- No references can outlive the data it points to.
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

- Dangling references are not allowed

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

Here, dangle() function returns a reference to a String. But the String is created inside the function and it goes out of scope when the function ends. So, the reference will be pointing to a memory location that is no longer valid

This will not work and give the error at return statement.
```rust
error[E0106]: missing lifetime specifier
```

> Lifetimes are a complex topic and will be covered in a separate note. Simply put, lifetimes are a way to ensure that references are valid for a certain scope of time.


### Pointer types in Rust

- Box<T> 
- Rc<T> 
- Arc<T> 
- Cell<T> 
- RefCell<T>

## Box\<T> - Heap Allocated Smart Pointer

Box<T> is a smart pointer that allows you to store data on the heap rather than the stack. It is used when you have a piece of data whose size can't be known at compile time and you want to use it in a context that requires an exact size.

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```
- Box<T> implements the `Deref` trait, which allows Box<T> values to be treated like references. `Box::new()` returns a Box<T> that points to the value 5. This can be dereferenced using `*` operator.

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
`x` is a value of type i32- `y` is a reference to `x` and `z` is a Box that points to `x`. Both points to the same value and are immutable.

Here `*y` and `*z` are dereferenced and compared. This is possible because Box<T> implements Deref trait.

- Box<T> also implements the `Drop` trait, which allows you to customize the code that is run when a Box<T> goes out of scope.

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

This code will print the following output. Because the Drop trait is implemented for CustomSmartPointer, the code in the drop method is run when the instances of CustomSmartPointer go out of scope.

```console
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

> Drop trait is used to implement the destructor for a type.

- Mutability of Box<T>

Box<T> is a smart pointer that points to a value on the heap. By default, the value is immutable. If you want to mutate the value, you should use `Box<T>` with `*mut` pointers. 

```rust
fn main() {
    let mut x = Box::new(5);
    *x = 10;
    println!("x: {}", x);
}
```

Even though we need to use dereference with primitives, when we use `Box<T>` with structs or enums, we don't need to use dereference.

```rust 
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Box::new(Point { x: 5, y: 10 });
    println!("p.x: {}", p.x);
}
```

This will print the following output.

```console
p.x: 5
```

How this works is that `Box<T>` implements the `Deref` trait only for non-primitive data types, which allows `Box<T>` values to be treated like references. So, when we use `p.x`, it is automatically dereferenced and the value of `x` is returned. This is called _deref coercion_.

### Box\<T> Methods

- `Box::new()` - Creates a new Box that points to the value passed to it.

- `Box::as_ref()` - Returns a reference to the value inside the Box. Immutable reference is returned.

- `Box::as_mut()` - Returns a mutable reference to the value inside the Box.


## Rc\<T> - Reference Counted Smart Pointer (Immutable)

`Rc<T>` is a reference counting smart pointer. It keeps track of the number of references to a value and when the count goes to zero, the value is dropped. The count goes up when a new reference is created and goes down when a reference is dropped.

- `Rc<T>` is a pointer to a heap-allocated value of type T.

- Implements `Drop`, `Deref` and `Clone` traits.

- `Rc<T>` is used when you want to allocate a value on the heap and have multiple references to it.

- `Rc<T>` cannot be used to mutate the value it points to. If you want to mutate the value, you should use `RefCell<T>`.

- `Rc<T>` is not thread safe. If you want to use it in a multi-threaded environment, you should use `Arc<T>`.

We can check the count of references using `Rc::strong_count` method.

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("Count: {}", Rc::strong_count(&a));

    drop(b);
    println!("Count: {}", Rc::strong_count(&a));

    drop(a);
    println!("Count: {}", Rc::strong_count(&c));
}
```
This will print the following output.

```console
Count: 3
Count: 2
Count: 1
```

Here, `Rc::new(5)` creates a new Rc that points to the value 5. `Rc::clone(&a)` creates a new reference to the value that `a` points to. `Rc::strong_count(&a)` returns the number of references to the value that `a` points to. `drop(b)` drops the reference `b` and the count goes down to - `drop(a)` drops the reference `a` and the count goes down to 1. Even if we drop the starting `a` reference, the value is not dropped because `c` still has a reference to it.

### Rc\<T> Methods

- `Rc::new()` - Creates a new Rc that points to the value passed to it.

- `Rc::clone()` - Creates a new reference to the value that the Rc points to.

- `Rc::strong_count()` - Returns the number of references to the value that the Rc points to.

- `Rc::weak_count()` - Returns the number of weak references to the value that the Rc points to.

- `Rc::downgrade()` - Creates a new weak reference to the value that the Rc points to.

- `Rc::upgrade()` - Converts a weak reference to a strong reference.

## RefCell\<T>

`RefCell<T>` is used to mutate a value inside an immutable reference.

- `RefCell<T>' can be used inside an Rc<T> to mutate the value it points to. This way we can have multiple references to a value and still mutate it.

```rust
fn main()
{
    let x = Rc::new(5));

    println!("x: {}", x.borrow());
}

```

### Methods of RefCell\<T>

- `RefCell::new()` - Creates a new RefCell that contains the value passed to it.

- `RefCell::borrow()` - Returns an immutable reference to the value inside the RefCell.

- `RefCell::borrow_mut()` - Returns a mutable reference to the value inside the RefCell.











