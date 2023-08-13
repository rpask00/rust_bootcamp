// Fix the code so that it compiles.

fn main() {
    let mut str1 = String::from("Rust");
    let ref1 = &mut str1;

    // str1.push('🦀'); // teo
    ref1.push('🦀');

    println!("ref1 = {}", ref1);
    println!("str1 = {}", str1);
}
