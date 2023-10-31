use std::sync::{Arc, Mutex};

struct Database {
    pub connections: Vec<u32>,
}

fn main() {
    let db = Database {
        connections: vec![]
    };

    // Mutex pozwala na zablokowanie zasobu wyłącznie dla jednego wątku, jednak żeby się do niego odwołać potrzebny jest Arc.
    let db  = Arc::new(Mutex::new(db));

    let mut handles = vec![];

    for i in 0..5 {
        let db = Arc::clone(&db);
        let h = std::thread::spawn(move || {
            let mut db_lock = db.lock().unwrap(); // MutexGuard obsługuje deref, więc można odwoływać się bezpośrednio do Database.

            db_lock.connections.push(i);

            // nie trzeba manualnie zamykać locka, bo zostanie on automatycznie zamknięty jak db_lock zostanie zwolniony
        });

        handles.push(h);
    }

    for h in handles{
        h.join().expect("Joining thread failed!");
    }

    println!("{:?}", db.lock().unwrap().connections);
}
