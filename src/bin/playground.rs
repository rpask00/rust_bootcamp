// Fix the print_details method. You can only modify the method body.

use std::cell::RefCell;
use rand::distributions::uniform::SampleBorrow;

struct Student {
    name: String,
    marks: u8,
    grade: RefCell<char>,
}

impl Student {

    fn new(name: &str, marks: u8) -> Self {
        Student {
            name: name.to_owned(),
            marks,
            grade: RefCell::new('X'),
        }
    }

    fn print_details(&self) {
        let grade = match self.marks {
            0..=33 => 'C',
            34..=60 => 'B',
            _ => 'A',
        };
        *self.grade.borrow_mut() = grade;
        println!(
            "name: {}, marks: {}, grade: {}",
            self.name,
            self.marks,
            self.grade.borrow()
        );
    }
}

fn main() {
   let x = Box::new(5);

    let student = Student::new("Harry", 70);
    student.print_details();
}
