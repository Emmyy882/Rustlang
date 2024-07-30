#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 2, y: 6 };
    let both_float = Point { x: 2.0, y: 6.4 };
    let integer_and_float = Point { x: 5, y: 12.1 };

    println!("point 1: {:?}", both_integer);
    println!("point 2: {:?}", both_float);
    println!("point 3: {:?}", integer_and_float);
}
