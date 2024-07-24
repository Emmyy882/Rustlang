fn main() {
    let v = vec![2, 4, 5, 6];

    println!("printing the result of the modified vector");
    for n_ref in &v {
        let n_plus_one = n_ref + 2;
        println!("{n_plus_one}");
    }

    println!("\nthe original vector = {:?}", v);
}
