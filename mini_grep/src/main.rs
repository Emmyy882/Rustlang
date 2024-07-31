use std::env;
use std::fs;

fn main() {
    // collecting the passed arguments into a vector list
    let args: Vec<String> = env::args().collect();

    /* dbg!(&args); // printing the passed arguments */

    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("should have been able to read to file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    // saving the command line argument values in variables
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
