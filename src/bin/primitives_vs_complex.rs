type TUPLE_INTEGERS = (i32, u32);
type TUPLE_W_STRING = (String, i32);
// -------------------------------------------
type ARRAY_INTEGERS = [i32; 3];
type ARRAY_W_STRING = [String; 3];
// -------------------------------------------
type ARRAY_TUPLE_INTEGERS = [TUPLE_INTEGERS; 3];
type ARRAY_TUPLE_W_STRING = [TUPLE_W_STRING; 3];

const x: i32 = 5;

fn main() {
    let ti: TUPLE_INTEGERS = (1, 19);

    print_TUPLE_INTEGERS(ti);
    print_TUPLE_INTEGERS(ti); // Tuple of primitives is primitive

    let ts: TUPLE_W_STRING = (String::from("Hello, world!"), 435);

    print_TUPLE_W_STRING(ts);
    // print_TUPLE_W_STRING(ts); // Tuple of complex types is complex, so it is moved

    let ai: ARRAY_INTEGERS = [1, 2, 3];

    print_ARRAY_INTEGERS(ai);
    print_ARRAY_INTEGERS(ai); // Array of primitives is primitive

    let as_: ARRAY_W_STRING = [
        String::from("Hello, world!"),
        String::from("Hello, world!"),
        String::from("Hello, world!"),
    ];

    print_ARRAY_W_STRING(as_);
    // print_ARRAY_W_STRING(as_); // Array of complex types is complex, so it is moved

    let ati: ARRAY_TUPLE_INTEGERS = [(1, 2), (3, 4), (5, 6)];
    print_ARRAY_TUPLE_INTEGERS(ati);
    print_ARRAY_TUPLE_INTEGERS(ati); // Array of tuples of primitives is primitive

    let ats: ARRAY_TUPLE_W_STRING = [
        (String::from("Hello, world!"), 1),
        (String::from("Hello, world!"), 2),
        (String::from("Hello, world!"), 3),
    ];

    print_ARRAY_TUPLE_W_STRING(ats);
    // print_ARRAY_TUPLE_W_STRING(ats); // Array of tuples of complex types is complex, so it is moved
}

fn print_TUPLE_INTEGERS(t: TUPLE_INTEGERS) {
    println!("{} {}", t.0, t.1);
}

fn print_TUPLE_W_STRING(t: TUPLE_W_STRING) {
    println!("{} {}", t.0, t.1);
}

fn print_ARRAY_INTEGERS(a: ARRAY_INTEGERS) {
    println!("{} {} {}", a[0], a[1], a[2]);
}

fn print_ARRAY_W_STRING(a: ARRAY_W_STRING) {
    println!("{} {} {}", a[0], a[1], a[2]);
}

fn print_ARRAY_TUPLE_INTEGERS(a: ARRAY_TUPLE_INTEGERS) {
    println!("{} {}", a[0].0, a[1].1);
    println!("{} {}", a[0].0, a[1].1);
    println!("{} {}", a[0].0, a[1].1);
}

fn print_ARRAY_TUPLE_W_STRING(a: ARRAY_TUPLE_W_STRING) {
    println!("{} {}", a[0].0, a[1].1);
    println!("{} {}", a[0].0, a[1].1);
    println!("{} {}", a[0].0, a[1].1);
}
