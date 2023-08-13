fn main() {
    let mut str1 = String::from("Rust");
    let ref1 = &mut str1; // teoretycznie istenieją 2 mutowalne referencje do str1

    // str1.push('🦀'); // ale w praktyce jeśl żyje ref1 to str1 nie można użyć
    ref1.push('🦀');

    println!("str1 = {}", str1); // jeśli ref1 nie jest już używane w przyszłości to można użyć str1
    // println!("ref1 = {}", ref1); // po odkomentowaniu tego linia wyżej przestanie działać
}

// wszystko tutaj zależy od lifetime'ów
