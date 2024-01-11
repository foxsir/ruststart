#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) {
    match coin {
        Coin::Penny => {
            println!("{}", 1)
        },
        Coin::Nickel => {
            println!("{}", 5);
        },
        Coin::Dime => {
            println!("{}", 10)
        },
        Coin::Quarter => {
            println!("{}", 25)
        },
    }
}

pub fn run() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 10;
    if let xcount = count {
        println!("The maximum is configured to be, {}", xcount)
    }

    println!("{}", count)

}
