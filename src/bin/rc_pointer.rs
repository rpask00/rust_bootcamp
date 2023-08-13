use std::rc::Rc;

fn main() {
    let v = 34;
    let r1 = Rc::new(v);

    // TODO: whats the difference between new and from?
    let r2 = Rc::from(v); // this does the same, dunno

    let r3 = r1.clone(); // this will work, because r1 is cloned
    let r4 = Rc::clone(&r1); // this will work, because r1 is cloned

    println!("r1 = {}", r1);
    println!("r3 = {}", r3);
    println!("r4 = {}", r4);
}
