enum IpAddrKind {
    V4,
    V6
}

struct IpAddr{
    kind : IpAddrKind,
    address : String
}

//enum without any struct 
#[derive(Debug)]
enum NewIpAddrKind {
    V4(String),
    V6(String)
}

impl NewIpAddrKind{
    fn get_ip(&self){
        println!("{:?}",&self);
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr{
        kind : IpAddrKind::V4,
        address : String::from("127.36.36.12")
    };

    let loopback = IpAddr{
        kind : IpAddrKind::V6,
        address : String::from("::1"),
    };

    let office = NewIpAddrKind::V4(String::from("127.69.69.75"));
    println!("{:?}",office.get_ip());
}


