#[derive(Debug)]
struct Button {
    text: String,
}


struct Container {
    text: String,
    child: Option<Box<Container>>,
}

fn main() {
    let button1 = Button { text: String::from("Button 1") }; // stored on stack

    let mut button2 = Box::new(Button { text: String::from("Button 2") }); // stored on heap - this is main point of using Box

    let button_ref = &mut button2;

    (*button_ref).text.push_str(" abc"); // example of automatic deref in Rust
    (**button_ref).text.push_str(" abc"); // example of automatic deref in Rust

    //  this means that b is the only owner of the value 10

    println!("b = {:?}", *button_ref);


    let c = Container {
        text: String::from("Container 1"),
        child: Some(Box::new(Container {
            text: String::from("Container 2"),
            child: None,
        })),
    };
}
