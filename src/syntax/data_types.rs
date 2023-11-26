pub fn run() {
  let guess: isize = "-2".parse().expect("Not a number!");

  println!("{}", guess);

  println!("Decimal: {}", 98_222);
  println!("Hex: {}", 0x_ff);
  println!("Octal: {}", 0o_77);
  println!("Binary: {}", 0b_1111_0000);
  println!("Byte (u8 only): {}", b'a');

  for x in b'A'..=b'B' {
    println!("{} as {}", x, x as char)
  }

  let heart_eyed_cat = '😻';

  println!("{}", heart_eyed_cat);
  println!("{}, length {}", '😻', "😻".len());

  // compound types
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  println!("{} : {} : {}", tup.0, tup.1, tup.2);
  println!("{:?}", tup.clone());

  let tup = (500, 6.4, 1);

  let (_x, y, _z) = tup;

  println!("The value of y is: {y}");

  let a = [1, 2, 3, 4, 5];

  println!("{}", a.len());
  println!("{:?}", &a[0..3]);

  let a2 = [2; 10];

  println!("{:?}", a2);
}
