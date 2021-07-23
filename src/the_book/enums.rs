#![allow(dead_code, unused_variables)]

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // We attach data to each variant of the enum directly
    V6(String),
}

enum Message {
    Quit,                       // has no data associated with it at all.
    Move { x: i32, y: i32 },    // includes an anonymous struct inside it.
    Write(String),              // includes a single String.
    ChangeColor(i32, i32, i32), // includes three i32 values.
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn run() {
    // enums_discover();
    // option_discover();
    if_let_discover();
}

fn if_let_discover() {
    // The if let syntax lets you combine if and let into lesser verbose syntax to handle values that match one pattern while ignoring the rest.
    // Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces.

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // is the same as
    if let Some(3u8) = some_u8_value {
        println!("three");
    } else {
        // handle the rest of the cases if there was a need for it
    }
}

fn option_discover() {
    // option<T> with match
    // when declare a Option<T> variable as None variant we must annotate it's data type since the compiler can't know what the data will be put into it
    // while when using the Some variant we don't have to annotate it as it will be known by the initialization value

    let mut option: Option<u8> = None;
    match option {
        None => println!("Null value!!!"),
        Some(i) => println!("Has a value of {}", i),
    }

    option = Some(5);
    match option {
        None => println!("Null value!!!"),
        Some(i) => println!("Has a value of {}", i),
    }

    // The _ pattern will match any value. By putting it after our other arms,
    // The _ will match all the possible cases that aren't specified before it.
    // The () is just the unit value
}

fn enums_discover() {
    let home = IpAddr::V4(0, 0, 0, 255);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("four: {:?}", home);
    println!("six: {:?}", loopback);

    //let c = Coin::Quarter(UsState::Alabama);
    let c = Coin::Penny;
    let value = value_in_cents(c);
    println!("the value is {}", value);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
