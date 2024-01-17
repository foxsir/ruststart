use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // --snip--

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            // println!("The secret number will be between 1 and 100.");
            panic!("Guess value must be between 1 and 100, got {}.", guess);
        }


    }
}
