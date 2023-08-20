fn main() {
    let cats = "😼😼😼😼😼😼😼";

    let s1 = "Hello";
    let h = &s1[0..1];
    println!("{}", h); // this will work, because letter in utf-8 is 1 byte long

    let c = &cats[0..2];
    // println!("{}", c); // this will panic, because emoji in utf-8 is 4 bytes long
    let c = &cats[0..4];
    println!("{}", c); // this will work

    let b0 = cats.bytes().nth(0).unwrap();
    let c0 = cats.chars().nth(0).unwrap();

    println!("First byte of cats {}", b0);
    println!("First char of cats {}", c0);
}
