use std::collections::HashMap;
use std::env;

pub fn run () {
    let mut h = HashMap::new();

    h.insert("xx", "x");
    // h.remove("xx");

    println!("{:?}", h.get("xx"));
    println!("{}", h.get("xx").unwrap());
    println!("{:?}", Some("some"));

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Blue")).or_insert(100);

    println!("{:?}", scores.get(&String::from("Blue")));
    println!("{:?}", scores.get(&String::from("Blue")));
    println!("{}", scores.get(&String::from("Blue")).unwrap());
    println!("{:?}", scores);

    println!("---split---\n");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    println!("{:?}", env::current_dir().unwrap());

}
