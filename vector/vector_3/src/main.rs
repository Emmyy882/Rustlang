fn main() {
    let mut v = vec![2, 4, 6, 8, 9];
    let first = &v[0];

    v.push(6);
    println!("The first element is {}", first);
}
