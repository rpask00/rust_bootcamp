use std::fmt::Debug;
use serde::{Serialize};


#[derive(Debug, Serialize)]
struct BrowserCommand<T> {
    name: String,
    param: T,
}

impl<T: Debug> BrowserCommand<T> {
    fn new(name: String, param: T) -> Self {
        BrowserCommand { name, param }
    }

    fn print(&self) {
        println!("{:?}", self);
    }
}

impl BrowserCommand<i32> {
    fn double(&self) -> i32 {
        self.param * 2
    }
}

impl BrowserCommand<String> {
    fn capitalize(&self) -> String {
        self.param.to_uppercase()
    }
}


fn main() {
    let b1 = BrowserCommand::<String> {
        name: String::from("open"),
        param: String::from("https://www.google.com"),
    };

    let b2 = BrowserCommand::<i32> {
        name: String::from("zoom"),
        param: 123,
    };

    let b3 = BrowserCommand::new(
         String::from("scrool"),
            34
    );

    b1.print(); // this works on both types
    b2.print(); // this works on both types

    let c = b1.capitalize(); // capitalize is defined specifically for Strings
    // let c = b2.capitalize(); // this doesn't work on i32

    // let d = b1.double(); //this doesn't work on Strings
    let d = b2.double(); // double is defined specifically for i32

}
