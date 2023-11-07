use std::cell::RefCell;

fn main() {
    let rfc1 = RefCell::new(1);
    println!("RefCell1 - {}", rfc1.borrow());
    let rfc2 = RefCell::clone(&rfc1); // Zawartość RefCell też jest klonowana

    *rfc2.borrow_mut() = 2;


    println!("RefCell1 - {}", rfc1.borrow());
    println!("RefCell2 - {}", rfc2.borrow());
}
