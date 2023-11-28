pub fn run() {
  another_function(5);
  print_labeled_measurement(5, 'h');

  let y = {
    let x = 3;
    x + 1
  };

  println!("{}", five(100));

  println!("The value of y is: {y}");
}

fn five(x: i32) -> i32 {
  x + 5
}

fn another_function(x: u32) {
  println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value} {unit_label}");
}
