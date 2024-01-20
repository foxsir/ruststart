struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn run() {
    println!("first word: {}", first_word("word press"));
    // println!("lifetime")

    println!("{:?}", "word press".as_bytes().iter().enumerate());
    println!("{:?}", "word press".as_bytes().iter());

    println!("snip");

    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            y
        }

        println!("{}", longest("x", "y"));
    }

    {
        fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
            y
        }

        println!("{}", longest("x", "y"));
    }


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i.part);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", "press");
            return &s[0..i];
        }
    }

    &s[..]
}
