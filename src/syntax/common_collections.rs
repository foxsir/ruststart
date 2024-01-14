pub fn run () {
    let mut v: Vec<String> = Vec::new();
    v.push(String::from("20"));
    v.push(String::from("30"));
    v.push(String::from("10"));
    v.push(String::from("40"));

    v.sort();

    println!("len: {:?}", v.is_empty());
    println!("len: {}", v.len());
    println!("len: {:?}", v.first().unwrap());
    println!("len: {:?}", v.last());


    let mut v = vec![1, 2, 3, 4, 5];

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    println!("---split---\n\n");


    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);


    v.push(6);
    let first = &v[v.len() - 1];

    println!("{first}");

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        println!("{i}");
    }

}
