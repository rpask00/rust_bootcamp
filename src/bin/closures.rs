use std::ptr::eq;

struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    // ############################ EXAMPLE 1 ######################################3
    let validator1 = |username: &str, password: &str| {
        !username.is_empty() && !password.is_empty()
    };
    // Fn - Immutably borrow variables in environment.
    // FnMut - Mutably borrow variables in environment. Can change environment.
    // FnOnce - Take ownership of variables in environment. Can only be called once.
    let validator2 = |username: &str, password: &str| {
        let weak_passwords = vec!["password123!".to_owned()];
        !username.is_empty() &&
            !password.is_empty() &&
            password.len() > 8 &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
            weak_passwords.iter().find(|&weak_password| weak_password == password).is_none()
    };


    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator1,
    };
    println!("{}", creds.is_valid());

    // ############################ EXAMPLE 2 ######################################3


    let x: fn(x: i32) -> i32 = |x| x + 1;
    // fn to jest typ
    // Fn, FnMut, FnOnce to traity które może implementować fn
    // dlatego ten zapis nie ma sensu
    // let x: Fn(x: i32) -> i32 = |x| x + 1;
    // bo Fn jest traitem, więc to tak jak by zapisać x: Paintable, gdzie Paintable to też trait
    // zamiast tego można zapisać w postaci trait object:
    let x: &dyn Fn(i32) -> i32 = &|x| x + 1;


    // ############################ EXAMPLE 3 ######################################3

    let factors = vec![100, 100, 200];


    let computer = |y: i32| {
        factors.iter().sum::<i32>()
    };

    // Funkcje przechowują referencje do zmiennych i obowiązują lifetime'y tak jak w obiektach
    // np. tutaj nie można użyć move, ponieważ computer trzyma referencje do factors, i jest użyty
    // poniżej deklaracji computer2
    // let computer2 = move |y: i32| {
    //     factors.iter().sum()
    // };

    println!("Computed value: {}", compute(12, &computer));
    // println!("Computed value: {}", compute(12, &computer2));

    // ############################ EXAMPLE 4 ######################################3
    let mut x = 10;
    // closure that mutates states needs to be mut -> this is what FnMut means
    let double = |a: i32| 2 * a;
    let mut multiply_x = |a| {
        x = a * x;
        return x;
    };


    multiply_x(4);


    let _ = compute(6, double);          // Fn can be passed as Fn
    let _ = compute_mut(6, double);      // Fn can be passed as FnMut
    // let _ = compute(6, multiply_x);               // FnMut cannot be passed as Fn
    let _ = compute_mut(6, multiply_x);  // FnMut can be passed to FnMut
    println!("{x}");

    // ############################ EXAMPLE 5 ######################################3

    let num = 6;
    let fact = factorial(num, decrement, multiply);
    println!("{num}! = {fact}");
}


// fn to concrete type, który może być podstawiony pod każdy trait: Fn, FnMut, FnOnce,
// ponieważ nie przechwytuje zmiennych z otoczenia
fn decrement(x: u32) -> u32 {
    x - 1
}

fn multiply(x: u32, y: u32) -> u32 {
    x * y
}

// fn pointer można podstawić pod każdy trait, ale też concrete type fn
fn factorial(num: u32, dec: impl Fn(u32) -> u32, mul: fn(u32, u32) -> u32) -> u32 {
    let mut res = 1;
    let mut temp = num;
    while temp > 1 {
        res = mul(res, temp);
        temp = dec(temp);
    }
    res
}


fn compute(x: i32, computer: impl Fn(i32) -> i32) -> i32 {
    computer(x)
}

fn compute_mut(x: i32, mut computer: impl FnMut(i32) -> i32) -> i32 {
    computer(x)
}

fn get_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + 6
}
