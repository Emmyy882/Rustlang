#[derive(Debug)]
// struct definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    /* creating user1 as an instance of User, and passing its paramaters
     * to the struct_builder function to return a complete User template
     */
    let user1 = struct_builder(
        String::from("Mike"),
        String::from("mike@gmail.com"),
    );

    println!("user details: {:#?}", user1);
}

fn struct_builder(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
