pub fn run() {

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    if number == 3 {
        println!("number was three");
    }

    // 在条件判断中使用返回值
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let number = String::from("");

    println!("The value of number is: {number}");

    // loop {
    //     std::io::stdin().read_line(&mut number).expect("error");
    //     println!("input was: {}", number);
    // }


    let mut counter = 0;

    // 在loop中使用返回值
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    for_label();
    use_while();

    let a = [10, 20, 30, 40, 50];

    a.iter().for_each(|second| {
        println!("{}", second)
    });

    let x = a.first();
    let xl = a.last();
    println!("{}", x.unwrap());
    println!("{:?}", xl.unwrap());

    println!("# 反转数组");
    for number in (1..=4).rev() {
        println!("{number}!");
    }

}


// 使用'+字符串给for命名
fn for_label() {
    'abc: for _ in 1..=10 {
        println!("break 'abc");
        break 'abc;
    }
}


// 使用 '+字符串给loop命名
fn _loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


fn use_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
