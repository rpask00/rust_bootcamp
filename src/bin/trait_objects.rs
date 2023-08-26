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
    green_object = get_green_object(false);

    paint_purple(&mut green_object);

    let green_object_ref1 = &mut green_object; // to zwróci mutowalną referencię do Boxa
    green_object_ref1.paint("purple".to_string());

    let green_object_ref2 = green_object.as_mut();  // to zwróci mutowalną referencję do obiektu, którego właścicielem jest ciągle Box
    green_object_ref2.paint("purple".to_string());

    let mut smth_paintable = create_paintable_house();
    smth_paintable.paint("red".to_string());

    let house = House {};
    let car = Car {};

    // let colection = vec![car, house]; // nie zadziała bo są różne typy
    // let collection: Vec<Box<dyn Paintable>> = vec![Box::new(car), Box::new(house)]; // zadziała bo są typy implementujące Paintable
    let collecition: Vec<&dyn Paintable> = vec![&car, &house];

    let h = ref_to_paintable(&mut green_object);
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

fn paint_purple(item: &mut Box<dyn Paintable>) {
    item.paint("purple".to_string());
}

fn ref_to_paintable() -> &'static dyn Paintable {
    if rand::random::<u8>() % 2 == 1 {
        return &House {};
    } else {
        return &Car {};
    }
}

fn ref_to_paintable2(item: &Box<dyn Paintable>) -> &dyn Paintable {
    if rand::random::<u8>() % 2 == 1 {
        return &House {};
    } else {
        return &Car {};
    }
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

fn create_paintable_house() -> impl Paintable {
    return House {};
}


// fn create_paintable_house3<T: Paintable>() -> T { // to nie zadziała
//     return House {};
// }

