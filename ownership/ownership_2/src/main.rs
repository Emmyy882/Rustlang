/* illustrating ownership in Rust using clone method to prevent moving of data to
 * another owner, in order for the previous reference to still be accessible
 */
fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone(); // cloning data which prevents moving of data
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
