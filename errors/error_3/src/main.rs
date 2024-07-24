use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt");

    let greeting_file = match greeting_file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("{:?}", greeting_file);
}
