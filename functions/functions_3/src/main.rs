//functions with return types
fn main() {
    // printing the value of the function directly
    println!("Printing the return value from function directly with.");
    println!("{}", return_func());

    // assigning the return vslue to a variable
    let x = return_func();
    println!("Printing the function return value assigned to a variable, x.");
    println!("x = {x}");
}

fn return_func() -> i32 {
    24 // the return value doesn't end with a semicolon
}
