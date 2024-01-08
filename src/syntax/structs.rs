struct User {
    _active: bool,
    _username: String,
    email: String,
    sign_in_count: u64,
}

pub fn run() {
    let user = User {
        _active: true,
        _username: String::from("scrum"),
        email: String::from("email"),
        sign_in_count: 10
    };
    println!("{} sign count {}", user.email, user.sign_in_count);

    let u2 = User {
        ..user
    };
    println!("{} sign count {}", u2.email, u2.sign_in_count);
}
