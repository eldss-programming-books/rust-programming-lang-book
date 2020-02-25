enum IpAddrKind {
    // enums can associate values with each subtype
    // each subtype can be associated with different kinds of data
    // these are basically tuple structs
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // no associated data
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // single String
    ChangeColor(i32, i32, i32), // three i32 values
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    // Can use the Option enum to represent empty vars
    // This is always in scope
    let some_num = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    print_opt_int(some_num);
    print_opt_int(absent_number);

    let dime = Coin::Dime;
    println!("A dime is worth {} cents", get_value(dime));

    print_odd_under_10(5);
    print_odd_under_10(6); // ignored
    print_odd_under_10(11); // ignored
}

fn print_opt_int(val: Option<i32>) {
    // Pattern matching can distinguish enum types
    match val {
        Some(x) => println!("{}", x),
        None => println!("Empty variable"),
    }
}

fn get_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn print_odd_under_10(num: u8) {
    match num {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        9 => println!("nine"),
        // catchall case, returns the "unit value", which does nothing
        _ => (),
    }
}
