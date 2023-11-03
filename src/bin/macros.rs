use std::collections::HashMap;

macro_rules! hello {
    () => {
        println!("Hello world");
    };
}

macro_rules! map {
    // $ [identifier] : [fragment-specifier]
    ($key:ty, $val:ty) => {
        {
            let map: HashMap<$key, $val> = HashMap::new();
            map
        }
    };
    // $ ( ... ) repetition syntax
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    };
}
fn main() {
    // [], {}, () to matching tokens, i można je używać zamiennie, jednak w przypadku vec, najwięcej sensu ma []
    hello!();
    hello![];
    hello! {}
    let a1 = vec![1, 2, 3];
    let a2 = vec!(1, 2, 3);
    let a3 = vec! {1, 2, 3};

    println!("{:?}", a1);
    println!("{:?}", a2);
    println!("{:?}", a3);

    let m1 = map!(String, i32);
    let m2 = map!(
        "RED" => 1,
        "GREEN" => 1,
        "BLUE" => 1
    );

    println!("{:?}", m1);
    println!("{:?}", m2);
}
