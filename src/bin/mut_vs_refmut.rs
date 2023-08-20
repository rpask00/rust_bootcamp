fn main() {
    let mut s1 = String::from("Hello");

    mut_borrow(&mut s1);
    mut_borrow(&mut s1);

    mut_move(s1);
    // mut_move(s1); // this won't work, because s1 is moved

    let mut x = 4;
    let mut y = &mut x;
}

fn mut_borrow(some_string: &mut String) {
    some_string.push_str(", world");
}

fn mut_move(mut some_string: String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}

fn mut_ref_to_mut_ref(mut some_string: &mut String) {
    println!("{}", some_string);
    some_string.push_str(", world");
}

fn mut_ref_to_mut_ref2(mut some_string: &mut String) {
    println!("{}", some_string);
    some_string.push_str(", world");
}
