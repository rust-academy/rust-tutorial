use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

#[derive(Debug, Clone)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
