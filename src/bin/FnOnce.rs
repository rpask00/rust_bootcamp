fn main() {
    // ############################ EXAMPLE 1 ######################################3
    let data = vec![1, 2, 3];

    let closure = move || { // to jest Fn, a nie FnOnce
        // closure staje się właścicielem data
        // jednak println! odwołuje się jedynie do referencji
        // dlatego może być wywołany wielokrotnie
        println!("Here's your data: {:?}", data);

        // kolejne wywołanie closure to tak jak by zrobić kolejny println!
        println!("Here's your data: {:?}", data);

    };
    // println!("Here's your data: {:?}", data); // to nie zadziała bo data zostało przeniesione do closure

    // jednak closure może być wywołany wielokrotnie
    closure();
    closure();

    // ############################ EXAMPLE 2 ######################################3
    let data2 = vec![1, 2, 3];
    let closure2 = || { // to faktycznie jest FnOnce
        // tutaj różnica jest taka że na data2 jest wywoływane into_iter()
        // które konsumuje wartość, i kolejne odwołanie nie jest możliwe
        for d in data2 {
            println!("Here's your data: {:?}", d);
        }

        // nawet w tym samym bloku kodu
        for d in data2 {
            println!("Here's your data: {:?}", d);
        }
    };

    closure2();
    closure2(); // dlatego tym bardziej 2 wywołanie closure nie jest możliwe
}
