#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("This takes the {:#?} variant of the IpAddrKind enum", four);
    println!("This takes the {:?} variant of the IpAddrKind enum", six);
}
