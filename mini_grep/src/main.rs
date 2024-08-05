use std::env;
use std::process;

use mini_grep::Config;

fn main() {
    // collecting the passed arguments into a vector list
    let args: Vec<String> = env::args().collect();

    /* dbg!(&args); // printing the passed arguments */

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
