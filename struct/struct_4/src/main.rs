// this program shows the change of ownership of data by borrowing (referencing)
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

    println!("Area of rectangle = {}", area(&rect1));
    println!("width and height of rectangle are {} and {}", rect1.width, rect1.height);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
