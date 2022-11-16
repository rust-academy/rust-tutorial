use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

mod ip_addr;
mod optional;

fn main() {
    println!("Hello, Enum!");

    test_ip_enum();
    test_optional_enum();
}

fn test_ip_enum(){
    let ip_four_localhost = Ipv4Addr::new(127, 0, 0, 1);
    let ip_six_localhost = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);

    let home = ip_addr::IpAddr::V4(ip_four_localhost);
    let home_v6 = ip_addr::IpAddr::V6(ip_six_localhost);

    println!("{:?}", home);
    println!("{:?}", home_v6);
}

fn test_optional_enum(){
    optional::test_optional_enum();
}

