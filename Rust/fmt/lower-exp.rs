use std::fmt::LowerExp;
use std::fmt::Result;
use std::fmt::Formatter;
struct Point2D{
    x:f64,
    y:f64
}

impl LowerExp for Point2D{
    fn fmt(&self, f:&mut Formatter<'_>) -> Result {
        write!(f, "({:.2e}, {:.2e})", self.x, self.y)
    }
}

fn main() {
    const point:Point2D = Point2D{
        x:12_345.6789,
        y:-12_345.6789,
    };
    println!("Point: {:e}", point);
}