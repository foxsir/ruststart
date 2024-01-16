use std::fs;
use std::fs::File;
use std::io::{self, Read};

pub fn run () {
    println!("{:?}", read_username_from_file());
    println!("{:?}", read_username_from_file2());
    println!("{:?}", read_username_from_file3());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(String::from(username.trim())),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello2.txt")?.read_to_string(&mut username)?;

    Ok(username)
}


fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello2.txt")
}
