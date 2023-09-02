fn main() {
    let data = vec![1, 2, 3];

    let closure = move || {
        // The closure takes ownership of `data`
        println!("Here's your data: {:?}", data);
    };
    println!("Here's your data: {:?}", data); // wont work

    // Call the closure
    closure();
    closure();

    let data2 = vec![1, 2, 3];

    let closure2 =  || {
        // The closure takes ownership of `data`
        for d in data2 {
            println!("Here's your data: {:?}", d);
        }

        for d in data2 {
            println!("Here's your data: {:?}", d);
        }
    };

    closure2();
    closure2(); // cant call it twice



}
