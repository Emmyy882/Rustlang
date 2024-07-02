fn main() {
    let s = String::from("Hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &str = &s;

    println!("printing the sliced strings \nstring1: {hello}");
    println!("string2: {world}");
    println!("string3: {s2}");
}
