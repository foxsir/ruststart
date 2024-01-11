#[derive(Debug)]
enum Suit {
    Heart(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("ss")
    }
}


#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

impl Option<i32> {
    fn abs() -> i32 {
        100
    }
}

pub fn run() {
    let s = Suit::Heart(String::from("red"));

    println!("{:?}", s);

    let color = Message::ChangeColor(255, 255, 255);

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = Option::Some(100);

    println!("{:?}", absent_number)
}
