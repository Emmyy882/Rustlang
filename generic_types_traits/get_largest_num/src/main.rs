// function to return the largest number in a list
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0]; // borrowing the first element on the list

    // iterating through the items on the list
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    // returning the largest item
    largest
}

fn main() {
    let number_list = vec![23, 50, 100, 10];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![33, 55, 70, 15, 2];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}
