/*
 * this program tries to open the file 'hello.txt' and if the action is...
 * ...unsuccessful, it tries to create the file. If the creation of the..
 * ......file fails, it then panics and terminates the program
 */
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating this file: {:?}", error);
            })
        } else {
            panic!("Problem opening this file: {:?}", error);
        }
    });

    println!("{:#?}", greeting_file);
}
