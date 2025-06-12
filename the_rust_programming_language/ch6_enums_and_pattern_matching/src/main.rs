fn main() {
    println!("Hello, world!");
}

enum IPAddressKind {
    V4,
    V6,
}

struct IPAddress {
    kind: IPAddressKind,
    address: String,
}

fn f1() {
    let four = IPAddressKind::V4;
    let six = IPAddressKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IPAddressKind) {}

fn f2() {
    let home = IPAddress {
        kind: IPAddressKind::V6,
        address: String::from("127.0.0.1"),
    };

    let loopback = IPAddress {
        kind: IPAddressKind::V4,
        address: String::from("::1"),
    };
}

enum IPAddress2 {
    V4(String),
    V6(String),
}

fn f3() {
    let home = IPAddress2::V4(String::from("127.0.0.1"));
    let loopback = IPAddress2::V6(String::from("::1"));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    }
}
