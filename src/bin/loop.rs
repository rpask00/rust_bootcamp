fn main() {
    let x = loop {
        break 10; // break can return value
    };

    println!("x = {}", x);

    'outer: loop {
        // loop can be labeled
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer; // break can be labeled
        }

        println!("This point will never be reached"); // this will never be reached
    }
}
