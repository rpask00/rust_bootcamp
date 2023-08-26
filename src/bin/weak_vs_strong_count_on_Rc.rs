use std::rc::{Rc, Weak};


fn main() {
    let s1 = Rc::new(134);
    println!("Strong = {}, Weak = {}", Rc::strong_count(&s1), Rc::weak_count(&s1));

    let w1 = Rc::downgrade(&s1);

    println!("Strong = {}, Weak = {}", Rc::strong_count(&s1), Rc::weak_count(&s1));

    {
        let s2 = w1.upgrade().unwrap(); // w1 can store None, so we need to unwrap it
        println!("Strong = {}, Weak = {}", Rc::strong_count(&s1), Rc::weak_count(&s1));

        let w2 = Rc::downgrade(&s2);
        println!("Strong = {}, Weak = {}", Rc::strong_count(&s1), Rc::weak_count(&s1));
    }
    println!("Strong = {}, Weak = {}", Rc::strong_count(&s1), Rc::weak_count(&s1));


    println!("Value of strong = {:?}", s1);

    match w1.upgrade() {
        Some(s) => println!("Value of weak = {:?}", s),
        None => println!("Value of weak = None"),
    }
}
