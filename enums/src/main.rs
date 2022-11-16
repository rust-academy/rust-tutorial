use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

mod ip_addr;
mod optional;
mod coin;
mod dice;
mod if_let;

fn main() {
    println!("Hello, Enum!");
    test_ip_enum();
    test_optional_enum();
    test_coin();
    test_dice();
    test_if_let();
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

fn test_coin(){
    let c1 = coin::Coin::Quarter(coin::UsState::Alaska);

    println!("{}", c1);

    let val = c1.value_in_cents();

    println!("Coin {} has value {}", c1, val);

}

fn test_dice(){
    dice::dice_roll();
}

fn test_if_let(){

    if_let::test_if_let();

    let mut count = 0;

    match coin {
        coin::Coin::Quarter(state) =>  println!("State quarter from {:?}!", state),
        _ => count +=1
    }

    if let Some(max) = config_max {println!("The maximum: {}", max);}


    if let coin::Coin::Quarter(state) = c {
        println!("State quarter from {:?}!", state)
    } else {
        count +=1
    }

}