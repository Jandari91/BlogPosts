use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;
struct Point2D{
    x:f64,
    y:f64
}

impl Display for Point2D{
    fn fmt(&self, f:&mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    const point:Point2D = Point2D{
        x:10.0,
        y:-10.0,
    };
    println!("Point: {}", point);
}