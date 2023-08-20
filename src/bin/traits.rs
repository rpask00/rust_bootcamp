struct VehicleInfo {
    name: String,
    color: String,
    fuel_level: u32,
}

struct Car {
    info: VehicleInfo,
}

struct Truck {
    info: VehicleInfo,
}

struct House {
    color: String,
}


impl Paintable for House {
    fn paint(&mut self, color: String) {
        self.color = color;
    }
}

trait Paintable {
    fn paint(&mut self, color: String);
}

trait Loadable {
    fn load(&mut self, amount: u32);
}

trait Vehicle: Paintable {  // Supertrait
    fn drive(&mut self);
}

impl Paintable for Car {
    fn paint(&mut self, color: String) {
        self.info.color = color;
    }
}

impl Vehicle for Car {   // to działa bo Paintable zostało zaimplementowane dla Car, gdyby nie było tego bloku wyżej to by rzuciło błąd
    fn drive(&mut self) {
        println!("Driving car");
    }
}


impl Paintable for Truck {
    fn paint(&mut self, color: String) {
        self.info.color = color;
    }
}

impl Loadable for Truck {
    fn load(&mut self, amount: u32) {
        println!("Loading {} tons of cargo", amount);
    }
}

fn main() {
    let mut c = Car {
        info: VehicleInfo {
            name: String::from("Car"),
            color: String::from("Red"),
            fuel_level: 0,
        }
    };

    let h = House {
        color: String::from("White"),
    };

    let mut t = Truck {
        info: VehicleInfo {
            name: String::from("Truck"),
            color: String::from("Green"),
            fuel_level: 0,
        }
    };

    c.paint(String::from("Blue"));


    paint_and_load2(&mut t, String::from("Yellow"), 100);
}


// Trait bounds

fn paint_red<T: Paintable>(item: &mut T) {      // Method #1
    item.paint(String::from("Red"));
}

fn paint_blue(item: &mut impl Paintable) {      // Method #2
    item.paint(String::from("Blue"));
}

fn paint_green<T>(item: &mut T)                 // Method #3
    where T: Paintable
{
    item.paint(String::from("Green"));
}


// Trait bounds with multiple traits

fn paint_and_load1<T: Loadable + Paintable>(item: &mut T, color: impl Into<String>, payload: u32) {    // Method #1
    item.paint(color.into());
    item.load(payload);
}


fn paint_and_load2(item: &mut (impl Loadable + Paintable), color: impl Into<String>, payload: u32) {    // Method #2
    item.paint(color.into());
    item.load(payload);
}


fn paint_and_load3<T>(item: &mut T, color: impl Into<String>, payload: u32)                             // Method #3
    where T: Loadable + Paintable
{
    item.paint(color.into());
    item.load(payload);
}
