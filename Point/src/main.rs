// structer representing a Point in geometry.
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// This method move this point to new postion.
    fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// This method return the distance between this point and other point.
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

fn main() {
    // create point_a as an instance of Point.
    let mut point_a = Point {
        x: 20.5,
        y: 15.0,
    };

    // move point_a to new postion.
    point_a.move_to(0.0, 0.0);

    // create point_b as an instance of Point.
    let point_b = Point {
        x: 30.0,
        y: 30.0,
    };

    // Print the distance between point_a and point_b
    println!("The distance between point_a and point_b is {}", point_a.distance(&point_b));
}
