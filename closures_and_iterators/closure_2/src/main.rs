fn main() {
    let list = vec![1, 2, 3];
    println!("before defining closure: {:?}", list);

    let only_borrow = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrow();
    println!("After calling closure: {:?}", list);
}
