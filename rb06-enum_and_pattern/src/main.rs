enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Wa,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        other => {
            println!("Wa");
            80
        }
    }
}

fn main() {
    println!("Hello, world!");
    let c = Coin::Wa;
    value_in_cents(c);
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    let config_max = Some(3u8);
    match config_max {
        Some(m) => println!("The maximum is configured to be {m}"),
        _ => (),
    }

}

