#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = route(IpAddrKind::V4);
    let six = route(IpAddrKind::V6);

    println!("This takes the {:#?} of the IpAddrKind", four);
    println!("This takes the {:#?} of the IpAddrKind", six);
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}
