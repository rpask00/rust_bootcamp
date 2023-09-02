use std::collections::HashMap;
use std::num::FpCategory::Nan;

// Provide the trait implementations and make the code execute successfully.
struct Employee {
    first_name: String,
    last_name: String,
    id: String,
}


struct EmployeeIter {
    state: Vec<String>,
}

// po EmployeeIter można iterować, bo implementuje Iterator

impl Iterator for EmployeeIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state.len() == 0 {
            return None;
        }
        return Some(self.state.remove(0));
    }
}

// natomiast Employee implementuje IntoIterator, co zamienia go na EmployeeIter
impl IntoIterator for Employee {
    type Item = String;
    type IntoIter = EmployeeIter;
    fn into_iter(self) -> Self::IntoIter {
        EmployeeIter {
            state: vec![
                self.first_name,
                self.last_name,
                self.id,
            ]
        }
    }
}

fn main() {
    // ############################ EXAMPLE 1 ######################################3
    let employee = Employee {
        first_name: "Alice".to_owned(),
        last_name: "Smith".to_owned(),
        id: "ab123".to_owned(),
    };
    let mut emp_iter = employee.into_iter(); // into_iter() zwraca EmployeeIter
    println!("First name: {}", emp_iter.next().unwrap());
    println!("Last name: {}", emp_iter.next().unwrap());
    println!("ID: {}", emp_iter.next().unwrap());
    assert_eq!(emp_iter.next(), None);

    // ############################ EXAMPLE 2 ######################################3

    let mut elements = HashMap::new();
    elements.insert("H".to_owned(), "Hydrogen".to_owned());
    elements.insert("He".to_owned(), "Helium".to_owned());
    elements.insert("Li".to_owned(), "Lithium".to_owned());

    // ---------------------------------- iter --------------------------------

    // iter iteruje po referencjach
    let iter = elements.iter();
    for (key, value) in iter {
        println!("iter: {:?} => {:?}", key, value);
    }
    // jest to to samo co:
    // for (key, value) in &elements {


    // ---------------------------------- iter_mut ----------------------------

    // iter_mut iteruje po mutowalnych referencjach (elements musi być oznaczone jako mut)
    let iter_mut = elements.iter_mut();
    for (key, value) in iter_mut {
        println!("iter_mut: {:?} => {:?}", key, value);
    }

    // jest to to samo co:
    // for (key, value) in &mut elements {}

    // ---------------------------------- into_iter ---------------------------

    // into_iter iteruje po own values
    let into_iter = elements.into_iter();
    for (key, value) in into_iter {
        println!("into_iter: {:?} => {:?}", key, value);
    }

    // jest to to samo co:
    // for (key, value) in elements {}


    // po czymś takim nie można się już odwołać do HashMapy, ponieważ jest ona moved do into_iter
    // elements.insert("Be".to_owned(), "Beryllium".to_owned());



}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| capitalize_first(s)).collect()
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    capitalize_words_vector(words).join("")
}
