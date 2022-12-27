fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p: Point<f32> = Point { x: 5.0, y: 10.0 };
    println!("distance from origin = {}", p.distance_from_origin());
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
