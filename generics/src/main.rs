// generics: abstract stand-ins for concrete types or other properties
fn main() {}
struct Shape {
    type: ShapeDef,
    area: u32,
    perimeter: u32,
}
enum ShapeDef{
    Square(width: u32),
    Triangle(width: u32,height: u32),
    Circle(radius: u32)
}