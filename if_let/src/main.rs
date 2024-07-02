#![allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn main() {
    let config_max = Some(3u8); // int types can be used as literal suffixes
    

    // this is functionally identical...
    if let Some(max) = config_max {
        println!("config_max: {max}")
    }

    // to this:
    match config_max {
        Some(val) => println!("config_max: {val}"),
        _ => (),
    }

    // convenient if we only care about one branch and want to discard all other cases
    // in a match statement
    let my_coin = Coin::Quarter(String::from("California"));
    if let Coin::Quarter(state) = my_coin {
        println!("Quarter from {}", state);
    } else { // and we can chain `else` onto it 
        println!("Some coin");
    }
}
