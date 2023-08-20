struct House;

struct Car;

trait Paintable {
    fn paint(&mut self, color: String);
}

impl Paintable for House {
    fn paint(&mut self, color: String) {
        println!("Painting house with {}", color);
    }
}

impl Paintable for Car {
    fn paint(&mut self, color: String) {
        println!("Painting car with {}", color);
    }
}


fn main() {
    let mut green_object = get_green_object(true);

    let green_object_ref1 = &mut green_object; // to zwróci mutowalną referencię do Boxa
    green_object_ref1.paint("purple".to_string());

    let green_object_ref2 = green_object.as_mut();  // to zwróci mutowalną referencję do obiektu, którego właścicielem jest ciągle Box
    green_object_ref2.paint("purple".to_string());
}


// Kompilator zdefiniuje sobie 2 funkcje przyjmujące konkretne typy, tak jak poniżej,
// dlatego będzie wiedział czyim tak na prawdę jest T
fn paint_yellow<T: Paintable>(mut item: T) -> T {
    item.paint("yellow".to_string());
    return item;
}

// Taka funkcja powstanie w momencie kompilacji
fn paint_yellow1(mut item: House) -> House {
    item.paint("yellow".to_string());
    return item;
}

// Taka funkcja powstanie w momencie kompilacji
fn paint_yellow2(mut item: Car) -> Car {
    item.paint("yellow".to_string());
    return item;
}


// Tutaj jest inaczej, bo w zależności od warunku funkcja może zwrócić zupełnie inne obiekty
// dlatego w momencie kompilacji, kompilator nie zna rozmiaru zwracanego obiektu,
// z tego właśnie powodu nie możemy zwrócić bezpośrednio implementacji Paintable:

// fn get_green_object(vehicle: bool) -> Paintable { // to nie zadziałą

// zamiast tego trzeba zwrócić pointer do obiektu (wielkość samego pointera będzie znana w momencie kompilacji)
fn get_green_object(vehicle: bool) -> Box<dyn Paintable> {
    let mut o: Box<dyn Paintable>;

    if vehicle {
        o = Box::new(Car {});
    } else {
        o = Box::new(House {});
    }

    o.paint("green".to_string());

    return o;
}
