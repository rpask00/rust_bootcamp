// Complete the code by addressing the TODO.

struct User {
    name: String,
    age: u8
}

fn main() {
    let user = User {
        name: String::from("Tom Riddle"),
        age: 17u8,
    };
    println!("User's name: {}", user.name);
    println!("User's age: {}", user.age);
}
