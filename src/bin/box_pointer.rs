fn main() {
    let mut b = Box::new(10);
    let b2 = &mut b;

    *b2 = 30; // this won't work, because b2 is a reference to pointer not to value
    **b2 = 30; // this will work

    //  this means that b is the only owner of the value 10

    println!("b = {}", *b);
}
