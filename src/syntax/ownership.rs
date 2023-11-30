pub fn run() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
    let x = 5;
    {
        println!("{}", x.to_string().parse::<i32>().unwrap());
    }
    let y = x;
    println!("Y: {}", y);
    println!("X: {}", x);

    let s1 = String::from("hello");
    let s2 = String::from("hello");

    println!("{}", s2);
    for x in s1.as_bytes() {
        println!("{}", x.clone() as char);
    }

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);


    let s = String::from("hello");
    ownership(s);
    println!("{}", s)
}



fn ownership(x: String) {
    println!("{}", x)
}
