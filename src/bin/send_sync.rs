use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

// !Send to oznaczenie że typ nie jest Send
// !Sync to oznaczenie że typ nie jest Sync

// Send oznacza że typ może być moved do innego wątku.
// Sync oznacza że można się odwołać do referencji typu w wielu wątkach (poprzez smart pointer np Arc).

fn main() {
    // Rc jest !Send i !Sync to znaczy że nie może być nawet moved do innego wątku,
    // wynika to z tego że może istnieć inny Rc z tym samym counterem.
    let rc = Rc::new(3);
    std::thread::spawn(move || {
        println!("{}", rc);
    });

    // RefCell jest Send co oznacza że może być moved do innego wątku.
    let refcell = RefCell::new(3);
    std::thread::spawn(move || {
        println!("{:?}", refcell);
    });

    // RefCell jest !Sync, co oznacza że nie można posiadać referencji do niego w kilku wątkach,
    // ma to sens bo RefCell trzyma wartość która może być mutowana.
    let refcell = Arc::new(RefCell::new(3));
    for _ in 0..3 {
        let refcell_copy = Arc::clone(&refcell);
        std::thread::spawn(move || {
            println!("{:?}", refcell_copy);
        });
    }
}
