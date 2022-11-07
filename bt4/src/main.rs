
fn main() {
    let pos = Position::Two;
    println!("pos {:?}", pos);

    let ip = IP::IPv4("127.0.0.1".to_string());
    ip.get_ipv4();
}

#[derive(Debug)]
enum Position {
    One,
    Two,
    Three
}

#[derive(Debug)]
enum IP {
    IPv4(String),
    IPv6(String),
}

impl IP {
    fn get_ipv4(self) {
        match self {
            IP::IPv4(x) => println!("xxx: {}", x),
            _ => println!("failed")
        }
    }
}