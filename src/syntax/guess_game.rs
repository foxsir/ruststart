use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess() {
  loop {
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let range: i32 = rand::thread_rng().gen_range(1..=3);

    match guess.trim().parse::<i32>() {
      Ok(num) => num,
      Err(_) => {
        println!("Please entry digit!");
        continue;
      }
    };

    match range.cmp(&guess.trim().parse::<i32>().unwrap()) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
