use std::io;

fn main() {
  loop {
    println!("Input some number:");
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    let output = if input.trim() == "exit" {
      break;
    } else {
      let number: i32 = input.trim().parse().expect("Cannot parse string!");
      if number % 15 == 0 {
        "fizzbuzz".to_string()
      } else if number % 3 == 0 {
        "fizz".to_string()
      } else if number % 5 == 0 {
        "buzz".to_string()
      } else {
        number.to_string()
      }
    };
    println!("{}", output);
  }
}
