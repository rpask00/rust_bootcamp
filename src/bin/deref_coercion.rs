use std::ops::{Deref, DerefMut};


struct MyPointer<T> {
    value: T
}

impl<T> MyPointer<T>{
    fn new(value: T) -> MyPointer<T>{
        MyPointer {
            value
        }
    }
}
impl<T> Deref for MyPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MyPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}


fn main (){
    let s = Box::new(String::from("Alone in the room"));
    let my_s = MyPointer::new(s);
    let box_ref= my_s.deref();
    let str_ref = box_ref.deref();
    let str_slice_ref = str_ref.deref();

   // each works, mimo że przekazuje się coś innego za każdym razem
   // ale dzięki deref coercion zawsze otrzymuje sie &str
    deref_print(&my_s);
    deref_print(&box_ref);
    deref_print(&str_ref);
    deref_print(&str_slice_ref);

}


fn deref_print(value: &str) {
    println!("{value}");
}
