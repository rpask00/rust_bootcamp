use std::sync::mpsc;

// mpsc - Multi Producer Single Transmiter

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let band_names = vec![
        String::from("rehteeS"),
        String::from("srethgiF ooF"),
        String::from("kcablekciN"),
        String::from("noziroh eht em gnirB"),
        String::from("hcnup htaed regnif eviF"),
    ];


    for band_name in band_names {
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            let reversed = band_name.chars().rev().collect();
            tx_clone.send(reversed).expect("Receiver closed!");
        });
    }


    let first_name = rx.recv().unwrap(); // recv() blokuje wątek dopóki nie dostanie wartości, lub wszystkie transmittery są dropped.
    println!("First name returned: {}", first_name);

    drop(tx); // bez dropa głównego Transmittera wątek zatrzyma się na pętli poniżej.

    for r in rx { // Receiver można traktować jako iterator, ta pętla blokuje wątek dopóki istnieją transmittery.
        println!("{}", r);
    }
}
