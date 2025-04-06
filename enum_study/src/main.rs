#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

// Learning about match flow construct
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

//receives a coin from the type coin and returns a u8 number
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickelenny => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    println!("The value: {:?} {:?}", four, six);
}
