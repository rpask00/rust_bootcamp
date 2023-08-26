use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    value: RefCell<i32>, // RefCell is a container that allows mutable borrows checked at runtime
}

struct AuthService {
    db: Rc<Database>,
}

struct FileService {
    db: Rc<Database>,
}

fn main() {
    let db = Rc::new(Database { value: RefCell::new(213) });
    let auth = AuthService {
        db: Rc::clone(&db),
    };

    let file = FileService {
        db: Rc::clone(&db),
    };

    let db = Rc::new(Database { value: RefCell::new(213) });

    let mut r1 = db.value.borrow_mut();
    let mut r2 = db.value.borrow_mut();

    *r1 = 45;
    *r2 = 4325; // this doesn't throw error on compile time, but it panics on runtime

    println!("auth = {:?}", auth.db.value.borrow());
}

