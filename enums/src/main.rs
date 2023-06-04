#[derive(Debug)]
enum Shape{
    Circle(usize),
    Square(usize),
    Triangle(usize, usize),
}
impl Shape{
    fn new() -> Shape{
        Shape::Circle(3)
    }
}
fn main() {
    let s = Shape::Circle(3);
    let c = Shape::new();

    println!("{:?}", c);

    let x = Some(4u8);
    if let Some(val) = x {
        println!("{}", val);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value(coin: Coin) -> usize {
    match coin{
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,        
    }
}