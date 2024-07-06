#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // creating instance of User template
    let user1 = User {
        active: true,
        username: String::from("Mike"),
        email: String::from("mike@gmail.com"),
        sign_in_count: 5,
    };

    println!("user details = {:#?}", user1);
    println!("\nprinting just the name of the user1. \nname: {}", user1.username);
}
