#[derive(Debug)]
struct Dog {
    age: i32,
}

static mut M_LICZBA: i32 = 0;
static LICZBA: i32 = 0;

fn main() {
    let x = 124;
    let p_x = &x;
    // ------- PRINTOWANIE ADRESÓW
    println!("p_x points to {:p}", p_x); // "0x7fff85c5b214"
    // co prawda to też wypisze ten sam adres
    println!("p_x points to {:?}", p_x); // "0x7fff85c5b214"
    // ale różnica pojawia się w przypadku innych obiektów np.
    let s = String::from("ADSF");
    println!("Raw pointer holds address: {:?}", &s); // to wypisze to, na co wskazuje referencja do s "ASDF"
    println!("Raw pointer holds address: {:p}", &s); // a to wypisze faktyczny adres, pod jakim s się znajduje. "0x7fff85c5b214"

    let raw_x = &x as *const i32;
    // Wypisanie samego adresu nie jest unsafe same w sobie
    println!("Raw pointer holds address: {:p}", raw_x);
    // Ale dereferencja już tak
    unsafe {
        println!("Raw pointer holds address: {}", *raw_x);
    }

    let mut d = Dog {
        age: 34
    };

    // *mut - mutowalny pointer
    // *const - immutable pointer

    let raw_c = &d as *const Dog;
    // mapuj mutowalną referencje do d na mutowalny pointer o typie Dog
    let mraw_c = &mut d as *mut Dog;
    unsafe {
        // W bloku unsafe można jednocześnie posiadać immutable i mutable pointer

        println!("{:?}", *raw_c);

        (*mraw_c).age = 56;
        println!("{:?}", *mraw_c);

        println!("{:?}", *raw_c);
    }


    // ------- UNSAFE FUNCTIONS

    let mut s = String::from("asdf");
    // funkcje, które używają unsafe wewnątrz siebie mogą być wywołane bezpośrednio
    safe_fun(&mut s);

    // unsafe func mogą być tylko wywołane w unsafe scope
    unsafe {
        unsafe_fun(&mut s);
    }

    // ------- STATIC VARIABLES

    println!("{}", LICZBA);
    // static mutable variables są traktowane jak zwykły mutowalny raw pointer (*mut)
    // dlatego dereferencja ich jest unsafe
    unsafe {
        println!("{}", M_LICZBA);
    }
}


fn safe_fun(s: &mut String) {
    let mraw_s = s as *mut String;
    unsafe {
        (*mraw_s).push_str("h");
        println!("{:?}", *mraw_s);
    }
}


// oznaczenie funkcji jako unsafe oznacza, że może ona być wywołana, tylko w bloku unsafe
// dodatkowo cały scope funkcji jest traktowany jako unsafe block, ale prawdopodobnie zostanie to zmienione w przyszłości
unsafe fn unsafe_fun(s: &mut String) {
    let mraw_s = s as *mut String;
    (*mraw_s).push_str("h");
    println!("{:?}", *mraw_s);
}


