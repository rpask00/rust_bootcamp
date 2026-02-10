use lazy_static::lazy_static;

// const jest aliasem dla wartości, która podczas kompilacji po prostu zostanie podstawiona w miejsce, gdzie const jest wykorzystywany
// typ consta musib być jawnie podany, bo jego rozmiar musi być znany przy kompilacji.
const PLAYER_AGE: u8 = 10;

// static jest adresem do pamięci, czyli wartość występuje raz w pamięci, a nie wielokrotnie w binarce jak const
// default approach should be to use const, unless you need to use large amount of data with single instance
static CASINO_NAME: &str = "The Venetian";
// static COMPANY_NAME: String = String::from("The Venetian co"); // to nie zadział bo String::from nie jest const

fn expensive_computation() -> String {
    println!("Computing expensive value...");
    String::from("expensive result")
}

lazy_static! {
    static ref EXPENSIVE: String = expensive_computation();
}


fn main() {
    println!("Program started");
    // expensive_computation() hasn't run yet

    println!("About to access EXPENSIVE");
    println!("{}", *EXPENSIVE); // <- "Computing expensive value..." prints here

    // type of EXPENSIVE is something like lazy_static::lazy::Lazy
    // which is type of smart pointer that triggers value computation on deref

    println!("Accessing again");
    println!("{}", *EXPENSIVE); // <- No computation, uses cached value
}

// const fn to taka funkcja, która może być wywoływana podczas KOMPILACJI!,
const fn fibonacci(n: u32) -> u32 {
    // CZEGO NIE MOŻE ROBIĆ:

    // ❌ Heap allocation
    // let vec = Vec::new();

    // ❌ Calling non-const functions
    // let s = format!("Hello {}", "world");

    // ❌ Floating point operations (mostly)
    // let f = 3.14 * 2.0;

    // ❌ Accessing mutable statics
    // static mut COUNTER: i32 = 0;
    // unsafe { COUNTER += 1; }

    // ❌ Random operations
    // use rand::random;
    // let r = random::<u32>();

    if n < 2 {
        return 1;
    }

    fibonacci(n - 2) + fibonacci(n - 1)
}
