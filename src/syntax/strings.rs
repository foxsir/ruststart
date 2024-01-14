pub fn run () {
    println!("{}", "x".to_string());
    println!("{}", "x".len());
    for x in "abc".split("") {
        println!("{x}")
    }

    let mut hello = String::from("xx");
    let s1 = String::from("xz");


    hello.push_str("x");

    let s3 = hello.clone() + &hello.clone();

    println!("{:?}", hello);
    println!("{:?}", s3);
    println!("{:?}", s1);

    println!("{}", format!("{s1}-{hello}-{s3}"));
    println!("{:?}", String::from(s1.as_bytes()[0] as char));

    println!("{}", &"xxx"[0..2]);

    let abc = "xxx";
    println!("{}", &abc[0..2]);

}
