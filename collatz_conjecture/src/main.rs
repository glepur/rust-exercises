use std::io;

fn main() {
  loop {
    println!("Input a number bigger than 1:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");
    if input.trim() == "exit" {
      break;
    }
    let mut number: u32 = match input.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Number cannot be negative");
        continue;
      }
    };
    if number <= 1 {
      println!("Number must be bigger than 1");
      continue;
    }
    let mut iterations = 0;
    loop {
      if number == 1 {
        break;
      } else if number % 2 == 0 {
        number = number / 2;
      } else {
        number = number * 3 + 1;
      }
      iterations += 1;
    }
    println!("Collatz conjecture was resolved in {} iterations", iterations);
  }
}
