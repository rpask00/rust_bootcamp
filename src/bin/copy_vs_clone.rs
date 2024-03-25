#[derive(Debug)]
pub struct PointNoCopy {
    pub x: f64,
}


#[derive(Debug, Clone, Copy)] // Copy wymaga Clone
pub struct PointCloneAndCopy {
    pub x: f64,
    // pub s: String, // Copy może być implementowany tylko dla prostych typów które można kopiować po przez kopiowanie bitów
}

#[derive(Debug, Clone)]
pub struct PointCloneOnly {
    pub x: f64,
}

fn test_move() {
    let p1 = PointNoCopy { x: 0. };
    let p2 = p1; // tutaj następuje move
    // println!("{:?} {:?}", p1, p2); // dlatego to nie działa
}
fn test_copy_and_clone() {
    let p1 = PointCloneAndCopy { x: 0. };
    // kopiowanie jest niejawne (implicit)
    let p2 = p1; // tutaj następuje kopiowanie
    println!("{:?} {:?}", p1, p2);
}

fn test_clone_only() {
    let p1 = PointCloneOnly { x: 0. };
    // let p2 = p1; // tutaj nie nastąpi kopiowanie
    // klonowanie jest jawne (explicit) przez wykonanie metody clone()
    let p2 = p1.clone(); // ale za to możemy sklonować
    println!("{:?} {:?}", p1, p2);
}

fn main() {}
