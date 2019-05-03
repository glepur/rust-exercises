use std::io;

fn main() {
  loop {
    println!("Input term in Fibonacci sequence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Failed to read line");
    let output = if input.trim() == "exit" {
      break;
    } else {
      let number = input.trim().parse()
        .expect("Cannot parse string!");
      fibonacci(number)
    };
    println!("Result for provided term is: {}", output);
  }
}

fn fibonacci (number: i32) -> i32 {
  if number == 1 {
    number
  } else {
    number + fibonacci(number - 1)
  }
}
