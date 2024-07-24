/*
 * this program panics as a result of accessing an index in vector beyond
 * beyond the range of valid indexes
 */
fn main() {
    let v = vec![2, 4, 6];
    println!("{}", v[99]);
}
