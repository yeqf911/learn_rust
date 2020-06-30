use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn run() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    println!("{:?}", v4);
    println!("{:?}", v6);

    let home = MyIpAddr::V4(String::from("127.0.0.1"));
    let loopback = MyIpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let home = Address::V4(127, 0, 0, 1);
    let loopback = Address::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(Ipv6Addr::new(1, 1, 1, 1, 1, 1, 1, 1));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let coin_value = value_in_cents(Coin::Penny);
    println!("{}", coin_value);

    let coin_value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", coin_value);

    let res = plus_one(Some(1));
    println!("{:?}", res.unwrap());

    let res = plus_one(None);
    println!("{:?}", res);

    let u8_value = 4u8;
    match u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("other"),
    };

    // if let
    let some_value = Some(8u8);
    if let Some(8) = some_value {
        println!("eight");
    }

    let mut count = 1;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State is {:?}", state);
    } else {
        count += 1;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State is {:?}", state);
            25
        }
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum MyIpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Address {
    V4(u8, u8, u8, u8),
    V6(String),
}
