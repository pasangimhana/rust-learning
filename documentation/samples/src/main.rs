fn main() {
    let mut x = Box::new(5);
    *x = 10;
    println!("x: {}", x);

    x = 23;
    println!("x: {}", x);
}