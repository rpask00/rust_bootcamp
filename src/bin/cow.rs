use std::borrow::Cow;

fn main() {
    // Cow - copy on write - trzyma niemutowalną referencje do danych,
    // jednak gdy istnieje potrzeba to klonuje dane i dokonuje modyfikacji w kopii której jest właścicielem

    let bts = [1, -2, 3];
    let mut cow = Cow::Borrowed(&bts);

    // bts i cow wskazują na ten sam obszar w pamięci
    println!("{:?}", bts); // [1, -2, 3]
    println!("{:?}", cow); // [1, -2, 3]

    for i in 0..cow.iter().len() {
        if cow[i] < 0 {
            cow.to_mut()[i] = -cow.to_mut()[i];
        }
    }

    // bts i cow wskazują na różne obszary w pamięci
    println!("{:?}", bts); // [1, -2, 3]
    println!("{:?}", cow); // [1, 2, 3]
}
