pub fn run () {
    let s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);

    let s = String::from("hello world");

    let hello = &s[0..5];

    let len = s.len();

    println!("0..5: {}, 0..len: {}, length: {}", hello, &s[0..len], len);

    println!("{}", &s[0..1]);

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);


    // Other Slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, [2, 3]);

    println!("{:?}", slice);

    let mut s = String::from("x -> ");
    let s1 = &mut s;

    s1.push_str("1");

    {
        println!("{}", s)
    }

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
