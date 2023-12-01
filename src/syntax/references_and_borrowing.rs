pub fn run() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let _r1 = &mut s; // no problem

    let r2 = &mut s; // no problem

    println!("{}", r2);


    let mut s = String::from("hello");

    println!("============");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    r3.push_str("xxxx");
    println!("{}", r3);

    let _ = dangle();
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
