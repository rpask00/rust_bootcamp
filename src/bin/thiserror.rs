use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;




struct ExampleError {
    content: String,
}

impl Debug for ExampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ExampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.content.as_str())
    }
}

impl Error for ExampleError {}


// Kod powyżej można zastąpić tym - efekt jest ten sam
#[derive(Error, Debug)]
#[error("content")] // tutaj wskazujemy co ma zostać wypisane
struct ExampleThisError {
    content: String,
}


fn main() {
    match fallable_with_error() {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e)
    }

    match fallable_with_this_error() {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e)
    }
}


fn fallable_with_error() -> Result<i32, ExampleError> {
    let random = rand::random::<i32>();

    if random > 0 {
        return Ok(random);
    }

    return Err(ExampleError {
        content: "random greater than 0".to_string()
    });
}

fn fallable_with_this_error() -> Result<i32, ExampleThisError> {
    let random = rand::random::<i32>();

    if random > 0 {
        return Ok(random);
    }

    return Err(ExampleThisError {
        content: "random greater than 0".to_string()
    });
}
