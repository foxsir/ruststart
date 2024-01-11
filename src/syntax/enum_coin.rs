#[derive(Debug)]
enum UsState {
    Alabama,
    _Alaska,
    // --snip--
}

enum Coin {
    _Penny,
    // Nickel,
    // Dime,
    Quarter(UsState),
}

pub fn run() {
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        // match
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        // no match
        _ => count += 1,
    }

    println!("{}", count)
}
