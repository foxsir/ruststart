struct Square {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


pub fn run() {
    println!("{}", area(&Square {width: 30, height: 50}));


    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(square: &Square) -> u32 {
    square.width * square.height
}
