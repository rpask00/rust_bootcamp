use std::thread::sleep;
use std::time::Duration;


fn main() {

    std::thread::scope(|scope| {

        scope.spawn(|| {
            println!("Beginning of the first thread");
            sleep(Duration::from_secs(1));
            println!("End of the first thread");
        });

        scope.spawn(|| {
            println!("Beginning of the second thread");
            sleep(Duration::from_secs(2));
            println!("End of the second thread");
        });

        //  wątek spoza scope nie będzie brany pod uwagę
        // i zostanie ubity, jeśli pozostałe się zakończą
        std::thread::spawn(|| {
            println!("Beginning of the third thread");
            sleep(Duration::from_secs(3));
            println!("End of the third thread");
        })
    });

    // wykona się dopier po zakończeniu scope'owych wątków
    println!("After scope end");
}
