struct Rectangle{
    width: usize,
    height: usize,
}
impl Rectangle{
    fn area(&self) -> usize{
        self.width * self.height   
    }
}
fn main() {
    let r1 = Rectangle {width: 10,height: 10};

    println!("area is {}", r1.area());

}

fn area(r: &Rectangle) -> usize {
    r.width * r.height
}