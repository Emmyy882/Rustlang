// this program shows the change of ownership of data
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 25,
    };

    println!("Area of rectangle = {}", area(rect1));
    println!("width of rectangle = {}", rect1.width);
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
