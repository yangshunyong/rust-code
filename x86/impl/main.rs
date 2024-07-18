struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function (similar to a static method in C++)
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Method that operates on an instance of Point
    fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let mut p = Point::origin();
    println!("Distance to origin: {}", p.distance_to_origin());

    p.x = 3.0;
    p.y = 4.0;
    println!("Distance to origin: {}", p.distance_to_origin());

    let q = Point{x: 100.0, y: 200.0};
    println!("Distance to origin: {}", q.distance_to_origin());
}
